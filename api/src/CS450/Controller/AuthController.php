<?php

namespace CS450\Controller;

use CS450\Model\User;
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
        } catch (\InvalidArgumentException $e) {
            throw new Exception($e);
        }
        try {
            $payload = array(
                'token' => $this->user->register($userInfo),
            );
        } catch (\Exception $e) {
            throw new Exception($e);
        }
        

        return $payload;
    }
}
