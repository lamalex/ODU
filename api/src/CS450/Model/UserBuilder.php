<?php

namespace CS450\Model;

use CS450\Model;
use CS450\Service\DbService;

final class UserBuilder {
    private $db;

    public $id;
    public $name;
    public $email;
    public $role;
    public $password;
    public $department;

    /**
     * Annotation combined with phpdoc:
     *
     * @Inject
     * @param DbService $db
     */
    function __construct(DbService $db) {
        $this->db = $db;
    }

    function build() {
        return new User($this, $this->db);
    }

    function id($id) {
        $this->id = $id;
        return $this;
    }

    function name($name) {
        $this->name = $name;
        return $this;
    }

    function email($email) {
        $this->email = $email;
        return $this;
    }

    function role($role) {
        $this->role = $role;
        return $this;
    }

    function password($password) {
        $this->password = $password;
        return $this;
    }

    function department($department) {
        $this->department = $department;
        return $this;
    }
}
