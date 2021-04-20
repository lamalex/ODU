<?php
require_once __DIR__ . '/dbconfig.php';

use Appconfig\load_db_config;

use Psr\Container\ContainerInterface;
use function DI\factory;
use function DI\create;

use Monolog\Logger;
use Monolog\ErrorHandler;
use Monolog\Handler\StreamHandler;

use CS450\Model\UserBuilder;

use CS450\Service\DbService;
use CS450\Service\JwtService;
use CS450\Service\EmailService;

$db_conn_params = Appconfig\load_db_config(
    getenv("MYSQL_HOST"),
    getenv("MYSQL_USER"),
    getenv("MYSQL_PASSWORD"),
    getenv("MYSQL_DATABASE"),
);

$db_config = array_merge(
    $db_conn_params,
    array(
        'adapter' => 'mysql',
        'charset' => 'utf8',
        'port' => '3306',
    )
);

$jwt_config = json_decode(base64_decode(getenv("JWT_CONFIG")));

return [
    "env" => empty(getenv("PIPELINE_STAGE")) ? "development" : getenv("PIPELINE_STAGE"),
    "db" => $db_config,
    "jwt" => $jwt_config,
    UserBuilder::class => create()
        ->constructor(DI\get(DbService::class)),
    DbService::class => DI\Autowire(CS450\Service\Db\MysqlDb::class),
    JwtService::class => DI\Autowire(CS450\Service\Jwt\FirebaseJwt::class),
    EmailService::class => DI\Autowire(CS450\Service\Email\MailgunEmail::class),
    "mailgun.cfg" => (object) array(
        "apiKey" => getenv("MAILGUN_API_KEY"),
        "domain" => getenv("MAILGUN_DOMAIN"),
    ),
    Psr\Log\LoggerInterface::class => DI\factory(function () {
        $logger = new Logger("CS450");

        $fileHandler = new StreamHandler("php://stdout", Logger::DEBUG);
        $logger->pushHandler($fileHandler);

        ErrorHandler::register($logger);

        return $logger;
    }),
];
