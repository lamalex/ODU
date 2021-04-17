<?php

require_once __DIR__ . '/../../app/dbconfig.php';

use Psr\Container\ContainerInterface;
use function DI\factory;
use function DI\create;

use Monolog\Logger;
use Monolog\ErrorHandler;
use Monolog\Handler\StreamHandler;

use CS450\Service\DbService;
use CS450\Service\JwtService;


return [
    "BEST_FOOD" => "AVOCADO",
    "COOLEST_DOG" => "SNOOPY",
    "db" => [
        "host" => "mysql_for_tests",
        "user" => "cs450db_user",
        "pass" => "tomtit.TAD.inward",
        "name" => "cs450",
    ],
    "jwt" => (object) array(
        "kty" => "oct",
        "use" => "sig",
        "kid" => "sig-1618626246",
        "k" => "gY_qGJE9wkCKAxKbS5qZ6g",
        "alg" => "HS256",
    ),
    DbService::class => DI\Autowire(CS450\Service\Db\MysqlDb::class),
    JwtService::class => create(CS450\Service\Jwt\FirebaseJwt::class),
    Psr\Log\LoggerInterface::class => DI\factory(function () {
        $logger = new Logger("CS450-Test");

        $fileHandler = new StreamHandler(__DIR__ . "/logs/test.log", Logger::DEBUG);
        $logger->pushHandler($fileHandler);

        ErrorHandler::register($logger);

        return $logger;
    }),
];
