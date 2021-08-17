<?php declare(strict_types=1);

namespace CS450\Lib;

final class EmailAddress {
    private $email;

    public static function fromString(string $email): self {
        self::validateEmail($email);
        return new Self($email);
    }

    private function __construct(string $email) {
        $this->email = $email;
    }

    public function __toString(): string {
        return $this->email;
    }

    private static function validateEmail(string $email): void {
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            throw new \InvalidArgumentException(
                sprintf(
                    "%s is not a valid email address.",
                    $email
                )
            );
        } 
    }
}
