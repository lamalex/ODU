<?php 

namespace CS450\Model;

use CS450\Lib\Password;
use CS450\Lib\EmailAddress;
use CS450\Model\User\RegisterUserInfo;

final class User {
    /**
     * 
     * @Inject("jwt.key")
     */
    private $jwtKey;

    /**
     * 
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;

    /**
     * 
     * @Inject
     * @var CS450\Service\JwtService
     */
    private $jwt;

    /**
     * 
     * @Inject
     * @var CS450\Service\DbService
     */
    private $db;

    private function makeJwt($uid): string {
        $payload = array(
            'uid' => $uid,
        );

        return $this->jwt->encode($payload, $this->jwtKey);
    }

    public function login(EmailAddress $email, Password $password) {
        $conn = $this->db->getConnection();

        $selectEmailQ = "SELECT id, password FROM tbl_fact_users WHERE email=?";
        $stmt = $conn->prepare($selectEmailQ);

        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s, %s", $selectEmailQ, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "s",
            $email,
        ) && $stmt->execute();

        if (!$executed) {
            throw new \Exception($conn->error);
        }

        $stmt->bind_result($uid, $storedPassword);
        $stmt->fetch();

        $this->logger->debug(sprintf("verifying stored hash %s against new hash %s for user %d", $storedPassword, $password, $uid));

        if (!$password->verifyhash($storedPassword)) {
            throw new \Exception("Incorrect password");
        }

        $this->logger->info(sprintf(
            "User (%s) has been authenticated",
            $email
        ));

        return $this->makeJwt($uid);
    }

    public function register(RegisterUserInfo $userInfo): string {
        $insertUserSql = "INSERT INTO tbl_fact_users (name, email, password, department) VALUES (?, ?, ?, ?)";

        $conn = $this->db->getConnection();
        $stmt = $conn->prepare($insertUserSql);

        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s - %s", $insertUserSql, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "sssd",
            $userInfo->name,
            $userInfo->email,
            $userInfo->password,
            $userInfo->department,
        ) && $stmt->execute() && $stmt->close();

        if (!$executed) {
            if (Self::errorIsEmailExists($conn->error_list[0]["errno"])) {
                $this->logger->info(sprintf(
                    "Existing user found checking password %s %s",
                    $userInfo->email,
                    $userInfo->password,
                ));

                // check if passwords match.
                // -> if so log the user in
                // else -> redirect to login with error email exists
                return $this->login(
                    $userInfo->email,
                    $userInfo->password,
                );
            } else {
                // Something went wrong at the DB level
                throw new \Exception($conn->error);
            }
        }

        $uid = $conn->insert_id;
        $this->logger->info(sprintf("Created new user with id: %d", $uid));

        return $this->makeJwt(0);
    }

    private static function errorIsEmailExists(int $errorcode): bool {
        return $errorcode == 1062;
    }
}
