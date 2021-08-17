<?php

namespace CS450\Lib; 

class Exception extends \Exception {
    public function __construct(\Throwable $previous) {
        parent::__construct("Something went wrong on our end.", $previous->getCode(), $previous);

    }

    public function __toString(): string {
        $container = require __DIR__ . '/../../../app/bootstrap.php';
        $env = $container->get('env');

        $previous = $this->getPrevious();
        if ($env != "prod" && !empty($previous)) {
            $logger = $container->get(\Psr\Log\LoggerInterface::class);
            $logger->error($previous);

            $msg = $previous->getMessage();
        }

        return sprintf("%s %s", $this->getMessage(), $msg);
    }
}
