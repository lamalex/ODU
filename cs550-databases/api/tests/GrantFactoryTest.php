<?php declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Phinx\Wrapper\TextWrapper;
use Phinx\Console\PhinxApplication;
use Symfony\Component\Console\Input\StringInput;
use Symfony\Component\Console\Output\NullOutput;

use CS450\Model\GrantFactory;

final class GrantFactoryTest extends TestCase {
    private static $T;
    private static $db;
    private static $container;

    public static function setUpBeforeClass(): void {
        self::$container = require __DIR__ . '/testdata/bootstrap.php';
        self::$db = self::$container->get(CS450\Service\DbService::class);
    }

    protected function setUp(): void{
        $app = new PhinxApplication();
        $app->setAutoExit(false);
        $app->run(new StringInput(" "), new NullOutput());
    
        self::$T = new TextWrapper($app);
        self::$T->getMigrate("testing");
        self::$T->getSeed("testing");
    }
    
    protected function tearDown(): void{
        self::$T->getRollback("testing");
    }

    public function testFindAllGrants(): void {
        $userFactory = self::$container->get(CS450\Model\GrantFactory::class);
        $result = $userFactory->findAll();

        $this->assertEquals(
            5,
            count($result),
        );
    }
}
