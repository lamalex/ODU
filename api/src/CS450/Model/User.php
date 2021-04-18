<?php 

namespace CS450\Model;

use CS450\Lib\Password;
use CS450\Lib\EmailAddress;
use CS450\Model\User\RegisterUserInfo;

final class User {
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

    private function makeJwt($uid, $role): string {
        $payload = array(
            'uid' => $uid,
            'role' => $role,
        );

        return $this->jwt->encode($payload);
    }

    public function login(EmailAddress $email, Password $password) {
        $conn = $this->db->getConnection();

        $selectEmailQ = "SELECT id, password, user_role FROM tbl_fact_users WHERE email=?";
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

        $stmt->bind_result($uid, $storedPassword, $role);
        $stmt->fetch();

        $this->logger->debug(sprintf("verifying stored hash %s against new hash %s for user %d", $storedPassword, $password, $uid));

        if (!$password->verifyhash($storedPassword)) {
            throw new \Exception("Incorrect password", 69);
        }

        $this->logger->info(sprintf(
            "User (%s) has been authenticated with role %s",
            $email,
            $role,
        ));

        return $this->makeJwt($uid, $role);
    }

    public function register(RegisterUserInfo $userInfo): string {
        $role = 'FACULTY';
        $insertUserSql = "INSERT INTO tbl_fact_users (name, email, password, department, user_role) VALUES (?, ?, ?, ?, '$role')";

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

        return $this->makeJwt($uid, $role);
    }

    private static function errorIsEmailExists(int $errorcode): bool {
        return $errorcode == 1062;
    }
}
