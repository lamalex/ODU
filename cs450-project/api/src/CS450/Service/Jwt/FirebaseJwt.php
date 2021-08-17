<?php

namespace CS450\Service\Jwt;

use Firebase\JWT\JWT;
use CS450\Service\JwtService;

class FirebaseJwt implements JwtService {

    /**
    *
    * @Inject("jwt")
    */
    private $jwt;

    public function encode($payload) {
        return JWT::encode($payload, $this->jwt->k, $this->jwt->alg, null);
    }

    public function decode($jwt) {
        try {
            return (array) JWT::decode($jwt, $this->jwt->k, [$this->jwt->alg]);
        } catch (\UnexpectedValueException $e) {
            throw new InvalidTokenException($e);
        }
    }
}
