<?php declare(strict_types=1);
use PHPUnit\Framework\TestCase;

use CS450\Lib\Password;

final class PasswordTest extends TestCase {
    public function testCanBeCreatedFromValidPassword(): void {
        $this->assertInstanceOf(
            Password::class,
            Password::fromString("hot-dog-daddy-99")
        );
    }
    public function testModifiesGivenPassword(): void {
        $this->assertNotEquals(
            "hot-dog-daddy-99",
            Password::fromString("hot-dog-daddy-99")
        );
    }

    public function testRejectsEmptyPassword(): void {
        $this->expectException(InvalidArgumentException::class);

        Password::fromString("");
    }

    public function testRejectsPasswordShorterThan4(): void {
        $this->expectException(InvalidArgumentException::class);

        Password::fromString("123");
    }
}
