<?php declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use Phinx\Wrapper\TextWrapper;
use Phinx\Console\PhinxApplication;
use Symfony\Component\Console\Input\StringInput;
use Symfony\Component\Console\Output\NullOutput;

use CS450\Model\Grant;

final class GrantTest extends TestCase {
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

    public function testCreateGrant(): void {
        $grant = (new Grant(self::$db))
            ->setId(1)
            ->setTitle("Test Grant")
            ->setGrantNumber("TEST-1")
            ->setSourceId(1)
            ->setOriginalAmount(0)
            ->setBalance(0)
            ->setAdminId(1);

        $this->assertEquals(1, $grant->getId());
        $this->assertEquals("Test Grant", $grant->getTitle());
        $this->assertEquals("TEST-1", $grant->getGrantNumber());
        $this->assertEquals(1, $grant->getSourceId());
        $this->assertEquals(0, $grant->getBalance());
        $this->assertEquals(0, $grant->getOriginalAmount());
        $this->assertEquals(1, $grant->getAdminId());
    }

    public function testWritesOnSave(): void {
        $grant = (new Grant(self::$db))
            ->setTitle("Test Grant")
            ->setGrantNumber("TEST-1")
            ->setSourceId(1)
            ->setOriginalAmount(0)
            ->setBalance(0)
            ->setAdminId(1)
            ->save();

        $result = self::$db->getConnection()->query("SELECT * FROM tbl_fact_grants WHERE grant_number='TEST-1'");
        $data = $result->fetch_assoc();

        $this->assertEquals($grant->getTitle(), $data["title"]);
        $this->assertEquals($grant->getGrantNumber(), $data["grant_number"]);
        $this->assertEquals($grant->getSourceId(), $data["source_id"]);
        $this->assertEquals($grant->getOriginalAmount(), $data["original_amt"]);
        $this->assertEquals($grant->getBalance(), $data["balance"]);
        $this->assertEquals($grant->getAdminId(), $data["administrator_id"]);
    }
}
