<?php declare(strict_types=1);

use PHPUnit\Framework\TestCase;

use CS450\Lib\Password;
use CS450\Lib\EmailAddress;
use CS450\Model\UserFactory;

final class UserFactoryTest extends TestCase {

    private static $db;
    private static $container;

    public static function setUpBeforeClass(): void {
        self::$container = require __DIR__ . '/testdata/bootstrap.php';
        self::$db = self::$container->get(CS450\Service\DbService::class);
    }

    protected function setUp(): void {
        $conn = self::$db->getConnection();
        $result = $conn->query("SET FOREIGN_KEY_CHECKS = 0");
        $result = $conn->query("TRUNCATE TABLE tbl_fact_users");
        $result = $conn->query(sprintf(
            "INSERT INTO tbl_fact_users (name, email, password, department) VALUES ('%s', '%s', '%s', %d)",
            "Test User",
            "test@example.com",
            Password::fromString("TestPassword1"),
            1
        ));
        $this->assertTrue($conn->error === "", $conn->error);
    }

    protected function tearDown(): void
    {
        $conn = self::$db->getConnection();
        $result = $conn->query("SET FOREIGN_KEY_CHECKS = 0");
        $result = $conn->query("TRUNCATE TABLE tbl_fact_users");
        $this->assertTrue($result != false);
    }

    public function testReturnsFoundUser(): void {
        $userFactory = self::$container->get(CS450\Model\UserFactory::class);
        $user = $userFactory->findByEmail(EmailAddress::fromString("test@example.com"));

        $this->assertNotNull($user);
    }

    public function testReturnsNullForUserNotFound(): void {
        $userFactory = self::$container->get(CS450\Model\UserFactory::class);
        $user = $userFactory->findByEmail(EmailAddress::fromString("not_in_db@example.com"));

        $this->assertNull($user);
    }
/*
    public function testLoginCreatesJwtWithGoodData(): void {
        $conn = self::$db->getConnection();
        $result = $conn->query(sprintf(
            "INSERT INTO tbl_fact_users (name, email, password, department) VALUES ('%s', '%s', '%s', %d)",
            "Test User",
            "test@example.com",
            Password::fromString("TestPassword1"),
            1
        ));
        $this->assertTrue($conn->error === "", $conn->error);

        $loginInfo = LoginUserInfo::create("test@example.com", "TestPassword1");

        $user = self::$container->get(CS450\Model\User::class);
        $jwt = $user->login($loginInfo->email, $loginInfo->password);
        $jwtService = self::$container->get(CS450\Service\JwtService::class);

        $this->assertTrue(
            array_key_exists(
                'uid',
                (array) $jwtService->decode($jwt),
            ),
        );

        $this->assertTrue(
            array_key_exists(
                'role',
                (array) $jwtService->decode($jwt),
            ),
        );
    }
    
    public function testThrowsWhenUserDoesNotExist(): void {
        $loginInfo = LoginUserInfo::create("test@example.com", "TestPassword1");

        $user = self::$container->get(CS450\Model\User::class);
        
        $this->expectException(\Exception::class);
        $jwt = $user->login($loginInfo->email, $loginInfo->password);
    }

    public function testRegisterCreatesJwtWithGoodData(): void {
        $jwtService = self::$container->get(CS450\Service\JwtService::class);
        $registerInfo = RegisterUserInfo::create("test", "hi@example.com", "Abc12345", 1);

        $user = self::$container->get(CS450\Model\User::class);
        $jwt = $user->register($registerInfo);

        $this->assertTrue(
            array_key_exists(
                'uid',
                (array) $jwtService->decode($jwt),
            ),
        );

        $this->assertTrue(
            array_key_exists(
                'role',
                (array) $jwtService->decode($jwt),
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
                (array) $jwtService->decode($jwt),
            ),
        );

        $this->assertTrue(
            array_key_exists(
                'role',
                (array) $jwtService->decode($jwt),
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
    */
}
