<?php declare(strict_types=1);
use PHPUnit\Framework\TestCase;

use CS450\Lib\Request;

final class RequestTest extends TestCase {
    protected function setUp(): void
    {
        $_SERVER['REQUEST_URI'] = '';
    }

    public function testCanBeCreated(): void {
        $_SERVER['REQUEST_METHOD'] = "GET";

        $this->assertInstanceOf(
            Request::class,
            new Request()
        );
    }

    public function testBodyEmptyOnGET() {
        // Given a get request
        $_SERVER['REQUEST_METHOD'] = "GET";
        $req = new Request();

        $this->assertEquals(
            $req->getBody(),
            ""
        );
    }

    public function testBodyOnPOST() {
        $_SERVER['REQUEST_METHOD'] = "POST";
        $_POST['field1'] = "ice cream";
        $_POST['field2'] = "🤷🏻 gotta test emoji amiright";
        $_POST['field3'] = "<html>gets stripped</html>";

        $req = new Request();

        $this->assertEquals(
            $req->getBody(),
            array(
                "field1" => "ice cream",
                "field2" => "🤷🏻 gotta test emoji amiright",
                "field3" => "gets stripped",
            )
        );
    }

    public function testJSONEmptyOnGET() {
        // Given a get request
        $_SERVER['REQUEST_METHOD'] = "GET";
        $req = new Request();

        $this->assertEquals(
            $req->getJSON(),
            []
        );
    }

    public function testJSONOnPOST() {
        // Given a get request
        $_SERVER['REQUEST_METHOD'] = "POST";
        $_SERVER['CONTENT_TYPE'] = "application/json";
        $req = new Request([], __DIR__ . "/testdata/jsonbody.json");

        $this->assertEquals(
            $req->getJSON(),
            array(
                "field1" => "ice cream",
                "field2" => "🤷🏻 gotta test emoji amiright",
                "arrays" => array(
                    "banana sandwiches", "hot dogs"
                ),
                "an object" => array(
                    "getting" => "cray"
                ),
            )
        );
    }

    public function testJSONOnPOSTComplexContentType() {
        // Given a get request
        $_SERVER['REQUEST_METHOD'] = "POST";
        $_SERVER['CONTENT_TYPE'] = "application/json;charset=UTF-8";
        $req = new Request([], __DIR__ . "/testdata/jsonbody.json");

        $this->assertEquals(
            $req->getJSON(),
            array(
                "field1" => "ice cream",
                "field2" => "🤷🏻 gotta test emoji amiright",
                "arrays" => array(
                    "banana sandwiches", "hot dogs"
                ),
                "an object" => array(
                    "getting" => "cray"
                ),
            )
        );
    }

    public function testEmptyForNonJSONOnPOST() {
        // Given a get request
        $_SERVER['REQUEST_METHOD'] = "POST";
        $_SERVER['CONTENT_TYPE'] = "text/html; charset=UTF-8";

        $req = new Request([], __DIR__ . "/testdata/jsonbody.json");

        $this->assertEquals(
            $req->getJSON(),
            []
        );
    }
}
