<?php

namespace CS450\Controller;

use CS450\Model\User;
use CS450\Model\User\LoginUserInfo;
use CS450\Model\User\RegisterUserInfo;
use CS450\Lib\Exception;
use CS450\Lib\EmailAddress;

/**
 * @codeCoverageIgnore
 */
class AuthController
{
    /**
     * @Inject("env")
     */
    private $env;

    /**
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;

    /**
     * @Inject
     * @var CS450\Model\User
     */
    private $user;

    /**
     * @Inject
     * @var CS450\Service\JwtService
     */
    private $jwt;

    /**
     * @Inject
     * @var CS450\Service\EmailService
     */
    private $email;

    /**
     * @Inject
     * @var CS450\Service\DbService
     */
    private $db;

    public function login($params)
    {
        $loginData = $params["post"];
        $this->logger->info($loginData["email"] . " is trying to login.");

        try {
            $loginInfo = LoginUserInfo::create(
                $loginData["email"],
                $loginData["password"],
            );

            return array(
                'token' => $this->user->login($loginInfo->email, $loginInfo->password),
            );
        } catch (\Exception $e) {
            $this->logger->error("caught error throwing new one");
            throw new Exception($e);
        }
    }

    public function register($params)
    {
        $registerData = $params["post"];
        $this->logger->info("Registering user with " . print_r($registerData, true));
     
        try {
            $userInfo = RegisterUserInfo::create(
                $registerData["name"],
                $registerData["email"],
                $registerData["password"],
                $registerData["department"],
            );

            $payload = array(
                'token' => $this->user->register($userInfo),
            );
        } catch (\Exception $e) {
            throw new Exception($e);
        }
        

        return $payload;
    }

    private static function hostname(){
        return sprintf(
          "%s://%s",
          isset($_SERVER['HTTPS']) && $_SERVER['HTTPS'] != 'off' ? 'https' : 'http',
          $_SERVER['SERVER_NAME'],
        );
      }

    public function sendInvite($params) {
        if (empty($params["token"]) || $params["token"]["role"] !== "ADMINISTRATOR") {
            throw new \Exception("You are not authorized to invite new faculty. Please talk to your administrator");
        }

        $senderUid = $params["token"]["uid"];
        $adminEmail = $this->db->getConnection()
            ->query("SELECT email FROM tbl_fact_users WHERE ID = $senderUid")
            ->fetch_object()
            ->email;

        $to = $params["post"]["email"];

        $this->email->sendFromTemplate(
            EmailAddress::fromString($to),
            "Welcome! Register with grant management",
            "registration_invitation",
            array(
                "department"  => "IT",
                "startup_amt" => $params["post"]["startupAmount"],
                "admin_email" => $adminEmail,
                "registration_url" => sprintf("%s/#/register/%s",
                    self::hostname(),
                    urlencode(base64_encode(json_encode(array(
                        "email" => $params["post"]["email"],
                        "name" => $params["post"]["name"],
                        "department" => $params["post"]["department"],
                    )))),
                ),
            ),
        );
    }
}
