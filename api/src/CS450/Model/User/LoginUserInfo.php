<?php

namespace CS450\Model\User;

use CS450\Lib\Password;
use CS450\Lib\EmailAddress;

/**
 * @codeCoverageIgnore
 */
final class LoginUserInfo {
    public $email;
    public $password;

    public static function create(string $email, string $password): ?self {
        $email = EmailAddress::fromString($email);
        $password = Password::fromString($password);

        return new Self($email, $password);
    }

    private function __construct(EmailAddress $email, Password $password) {
        $this->email = $email;
        $this->password = $password;
    }
}
