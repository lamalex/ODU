<?php declare(strict_types=1);

use PHPUnit\Framework\TestCase;

use CS450\Model\User;
use CS450\Model\User\RegisterUserInfo;

final class UserTest extends TestCase {

    private static $container;
    private static $db;

    public static function setUpBeforeClass(): void {
        self::$container = require __DIR__ . '/testdata/bootstrap.php';
        self::$db = self::$container->get(CS450\Service\DbService::class);
    }

    protected function tearDown(): void
    {
        $conn = self::$db->getConnection();
        $result = $conn->query("SET FOREIGN_KEY_CHECKS = 0;");
        $result = $conn->query("DELETE FROM tbl_fact_users WHERE 1=1;");
        $this->assertTrue($result != false);
    }

    public function testRegisterCreatesJwtWithGoodData(): void {
        $jwtService = self::$container->get(CS450\Service\JwtService::class);
        $registerInfo = RegisterUserInfo::create("test", "hi@example.com", "Abc12345", 1);

        $user = self::$container->get('CS450\Model\User');
        $jwt = $user->register($registerInfo);

        $this->assertTrue(
            array_key_exists(
                'uid',
                (array) $jwtService->decode($jwt,
                    self::$container->get('jwt')->k,
                    array('HS256')
                )
            ),
        );
    }

    public function testRegisterLogsInWhenRegisteringValidUser(): void {
        $jwtService = self::$container->get(CS450\Service\JwtService::class);
        $registerInfo = RegisterUserInfo::create("test", "hi@example.com", "Abc12345", 1);

        // Given a user is already registered
        $user = self::$container->get('CS450\Model\User');
        $user->register($registerInfo);
        
        // When user tries to register using the same username and password
        // that they previously registered with
        $jwt = $user->register($registerInfo);

        // Then they are logged in
        $this->assertTrue(
            array_key_exists(
                'uid',
                (array) $jwtService->decode($jwt,
                    self::$container->get('jwt')->k,
                    array('HS256')
                )
            ),
        );
    }

    public function testThrowsWhenRegisteredUserReregistersWithNewPassword(): void {
        $jwtService = self::$container->get(CS450\Service\JwtService::class);
        $registerInfo = RegisterUserInfo::create("test", "hi@example.com", "Abc12345", 1);

        // Given a user is already registered
        $user = self::$container->get('CS450\Model\User');
        $user->register($registerInfo);
        
        // When user tries to register using the same username and password
        // that they previously registered with
        $registerInfo2 = RegisterUserInfo::create("test", "hi@example.com", "ANewPwd", 1);

        $this->expectException(\Exception::class);
        $jwt = $user->register($registerInfo2);
    }
}
