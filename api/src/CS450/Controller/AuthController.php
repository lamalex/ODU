<?php

namespace CS450\Controller;

use CS450\Model\User;
use CS450\Model\User\LoginUserInfo;
use CS450\Model\User\RegisterUserInfo;
use CS450\Lib\Exception;

/**
 * @codeCoverageIgnore
 */
class AuthController
{
    /**
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;

    /**
     * 
     * @Inject
     * @var CS450\Model\User
     */
    private $user;

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
}
