<?php declare(strict_types=1);
use PHPUnit\Framework\TestCase;

use CS450\Lib\EmailAddress;

final class EmailAddressTest extends TestCase {
    public function testCanBeCreatedFromValidEmailAddres(): void {
        $this->assertInstanceOf(
            EmailAddress::class,
            EmailAddress::fromString("user+dingus2@woopwoop.net")
        );
    }

    public function testCannotBeCreatedFromInvalidEmailAddress(): void {
        $this->expectException(InvalidArgumentException::class);

        EmailAddress::fromString("this_is_invalid@xxx");
    }

    public function testCanBeUsedAsString(): void {
        $this->assertEquals(
            "bigdaddyflex@aol.com",
            EmailAddress::fromString("bigdaddyflex@aol.com")
        );
    }
}
