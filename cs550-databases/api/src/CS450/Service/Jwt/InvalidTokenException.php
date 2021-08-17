<?php
namespace CS450\Service\Jwt;

use CS450\Lib\Exception;

class InvalidTokenException extends Exception {
    public function __construct(\Throwable $previous) {
        parent::__construct($previous);
    }

    public function getMessage(): string {
        return "The authentication token provided was invalid";
    }
}
