<?php 

namespace CS450\Lib;

final class Request
{
    public $uri;
    public $params;
    public $method;
    public $contentType;

    /**
     *
     * @Inject
     * @var CS450\Service\JwtService
     */
    private $jwt;

    private $authHeader;
    
    private $dataInputFile;

    public function __construct($params = [], $input = "php://input")
    {
        $this->dataInputFile = $input;

        $this->params = $params;
        $this->uri = $_SERVER['REQUEST_URI'];
        $this->method = trim($_SERVER['REQUEST_METHOD']);
        $this->contentType = trim($_SERVER["CONTENT_TYPE"] ?? "");
        $this->authHeader = trim($_SERVER["HTTP_AUTHORIZATION"] ?? "");
    }

    public function getBody()
    {
        if ($this->method !== 'POST') {
            return '';
        }

        $body = filter_var_array($_POST, FILTER_SANITIZE_STRING, true);

        return $body;
    }

    public function getJSON()
    {
        if ($this->method !== 'POST') {
            return [];
        }

        if (strpos($this->contentType, 'application/json') !== 0) {
            return [];
        }

        // Receive the RAW post data.
        $content = trim(file_get_contents($this->dataInputFile));
        return json_decode($content, true);
    }

    public function getAuthToken(): array {
        if (preg_match('/Bearer\s(\S+)/', $this->authHeader, $matches)) {
            $tokenBase64 = $matches[1];
            return $this->jwt->decode($tokenBase64);
        }

        return [];
    }
}
