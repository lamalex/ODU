<?php

use CS450\Lib\Request;
use CS450\Lib\Response;
use CS450\Lib\Exception;
use CS450\Service\Jwt\InvalidTokenException;
use FastRoute\RouteCollector;

error_reporting(E_ALL ^ E_WARNING);

$container = require __DIR__ . "/../app/bootstrap.php";
$dispatcher = FastRoute\simpleDispatcher(function (RouteCollector $r) {
    $r->addGroup("/api", function (RouteCollector $r) {
        $r->addRoute("GET", "/", "CS450\Controller\HomeController");
        $r->addRoute("GET", "/grants", "CS450\Controller\GrantController");
        $r->addRoute("GET", "/departments", "CS450\Controller\DepartmentController");
        $r->addRoute("GET", "/students", "CS450\Controller\StudentController");

        $r->addGroup("/auth", function (RouteCollector $r) {
            $authControllerName = "CS450\Controller\AuthController";

            $r->addRoute("POST", "/login", [$authControllerName, "login"]);
            $r->addRoute("POST", "/register", [$authControllerName, "register"]);
            $r->addRoute("POST", "/sendinvite", [$authControllerName, "sendInvite"]);
            $r->addRoute("POST", "/employ", [$authControllerName, "employ"]);            
        });

        $r->addGroup("/admin", function (RouteCollector $r) {
            $r->addRoute("GET", "/faculty", ["CS450\Controller\AdminController", "getFaculty"]);
            $r->addRoute("DELETE", "/faculty/{id:\d+}", ["CS450\Controller\AdminController", "deleteFaculty"]);
            $r->addRoute("GET", "/grants", ["CS450\Controller\GrantController", "grantsForAdmin"]);
            $r->addRoute("POST", "/grant/{id:\d+}", ["CS450\Controller\GrantController", "updateGrantStatus"]);
        });
    });
});


$request = $container->get(CS450\Lib\Request::class);

$route = $dispatcher->dispatch($request->method, $request->uri);

switch ($route[0]) {
    case FastRoute\Dispatcher::NOT_FOUND:
        Response::withCode(404)->toJSON();
        break;

    case FastRoute\Dispatcher::METHOD_NOT_ALLOWED:
        Response::withCode(405)->toJSON();
        break;

    case FastRoute\Dispatcher::FOUND:
        $controller = $route[1];
        $request->params = $route[2];

        try {
            $data = array_merge_recursive(
                ["params" => $route[2]],
                ["post" => $request->getJSON()],
                ["token" => $request->getAuthToken()],
            );

            $res = $container->call($controller, [$data]);
            echo Response::ok()->toJSON($res);
        } catch (InvalidTokenException $e) {
            echo Response::withCode(401)->toJSON(array(
                'message' => strval($e),
                'code' => $e->getCode(),
            ));
            return;
        } catch (Exception $e) {
            echo Response::error()->toJSON(array(
                'message' => strval($e),
                'code' => $e->getCode(),
            ));
        } catch (\Exception $e) {
            $e = new Exception($e);
            echo Response::error()->toJSON(array(
                'message' => strval($e),
                'code' => $e->getCode(),
            ));
        }

        break;
}
