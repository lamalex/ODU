<?php 

namespace CS450\Model\User;

use CS450\Lib\Password;
use CS450\Lib\EmailAddress;

abstract class UserRole {
    const SUPERUSER = 1;
    const ADMINISTRATOR = 2;
    const FACULTY = 3;
}

/**
 * @codeCoverageIgnore
 */
final class RegisterUserInfo {
    public $name;
    public $email;
    public $password;
    public $department;
    public $created_at;
    public $role;

    public static function create(string $name, string $email, string $password, int $department, $role = UserRole::FACULTY): ?self {
        $email = EmailAddress::fromString($email);
        $password = Password::fromString($password);

        return new Self($name, $email, $password, $department, $role);
    }

    private function __construct(string $name, EmailAddress $email, Password $password, int $department, $role) {
        date_default_timezone_set("America/New_York");

        $this->role = $role;
        $this->name = $name;
        $this->email = $email;
        $this->password = $password;
        $this->department = $department;
        $this->created_at = date('d-m-Y h:i:s A');
    }
}
