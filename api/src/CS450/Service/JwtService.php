<?php
 
namespace CS450\Service;

interface JwtService {
    /**
     * Converts and signs a PHP object or array into a JWT string.
     * @param object|array  $payload    PHP object or array
     * @param string        $key        The secret key.
     * @param string        $alg        The signing algorithm.
     * @param array         $head       An array with header elements to attach
     *
     * @return string A signed JWT
     */
    public function encode($payload);

    /**
     * Decodes a JWT string into a PHP object.
     *
     * @param string                    $jwt            The JWT
     * @param string|array|resource     $key            The key, or map of keys.
     *                                                  If the algorithm used is asymmetric, this is the public key
     * @param array                     $allowed_algs   List of supported verification algorithms
     *                                                  Supported algorithms are 'ES256', 'HS256', 'HS384', 'HS512', 'RS256', 'RS384', and 'RS512'
     *
     * @return object The JWT's payload as a PHP object
     *
     * @throws InvalidArgumentException     Provided JWT was empty
     * @throws UnexpectedValueException     Provided JWT was invalid
     */
    public function decode($jwt);
}
