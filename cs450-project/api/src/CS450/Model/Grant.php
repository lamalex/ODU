<?php

namespace CS450\Model;

use CS450\Service\DbService;

final class Grant implements \JsonSerializable {
    private $db;

    private $id;
    private $title;
    private $status;
    private $balance;
    private $source_id;
    private $grant_number;
    private $original_amt;
    private $administrator_id;

    private $_entity_id;
    private $_entity_name;
    private $_entity_type;

    private $_admin_id;
    private $_admin_name;
    private $_admin_role;
    private $_admin_email;
    private $_admin_department;

    private $recipients = [];

    public function __construct(DbService $db) {
        $this->db = $db;
    }

    public function getId(): int {
        return $this->id;
    }

    public function getTitle() {
        return $this->title;
    }

    public function getGrantNumber() {
        return $this->grant_number;

    }

    public function getStatus() {
        return $this->status;
    }

    public function getAdminId() {
        return $this->administrator_id;
    }

    public function getSourceId() {
        return $this->source_id;
    }

    public function getBalance() {
        return $this->balance;
    }

    public function getOriginalAmount() {
        return $this->original_amt;

    }

    public function getRecipients() {
        return $this->recipients;
    }

    public function getSourceEntity() {
        return array(
            "id" => $this->_entity_id,
            "name" => $this->_entity_name,
            "type" => $this->_entity_type,
        );
    }

    public function getAdministrator() {
        return (new User($this->db))
            ->setId($this->_admin_id)
            ->setName($this->_admin_name)
            ->setEmail($this->_admin_email)
            ->setRole($this->_admin_role)
            ->setDepartment($this->_admin_department);
    }

    function startupGrant() {
        $oduSourceId = $this->db->getConnection()
            ->query("SELECT id FROM tbl_fact_granting_entity WHERE name='ODU'")
            ->fetch_object()
            ->id;

        $this->status = "APPROVED";
        $this->title = "Starup Fund";
        $this->source_id = $oduSourceId;
        $this->grant_number = "ODU-STARTUP";

        return $this;
    }

    function for(User $user): Self {
        array_push($this->recipients, $user);
        return $this;
    }

    function setId($id): Self {
        $this->id = $id;
        return $this;
    }

    function setTitle($title): Self {
        $this->title = $title;
        return $this;
    }

    function setGrantNumber($grantNumber): Self {
        $this->grant_number = $grantNumber;
        return $this;
    }

    function setSourceId($sourceId): Self {
        $this->source_id = $sourceId;
        return $this;
    }

    function setOriginalAmount($originalAmount): Self {
        $this->original_amt = $originalAmount;

        return $this;
    }

    function setBalance($balance): Self {
        $this->balance = $balance;
        return $this;
    }

    function setStatus($status): Self {
        $this->status = $status;
        return $this;
    }

    function setAdminId($adminId): Self {
        $this->administrator_id = $adminId;
        return $this;
    }

    public function save(): Self {
        $insertGrantSql = <<<EOD
            INSERT INTO tbl_fact_grants (grant_number, title, source_id, original_amt, balance, status, administrator_id) 
            VALUES (?, ?, ?, ?, ?, ?, ?)
        EOD;

        $conn = $this->db->getConnection();
        $stmt = $conn->prepare($insertGrantSql);
        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s - %s", $insertGrantSql, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "ssiddsi",
            $this->grant_number,
            $this->title,
            $this->source_id,
            $this->original_amt,
            $this->balance,
            $this->status,
            $this->administrator_id,
        ) && $stmt->execute() && $stmt->close();
        

        if (!$executed) {
            throw new \Exception($conn->error);
        }
        $this->id = $conn->insert_id;

        $insertUserGrantMapSql = <<<EOD
            INSERT INTO tbl_fact_map_grant_users (grant_id, user_id)
            VALUES (?, ?)
        EOD;

        $stmt = $conn->prepare($insertUserGrantMapSql);

        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s - %s", $insertUserGrantMapSql, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "ii",
            $this->id,
            $userId
        );

        foreach ($this->recipients as $user) {
            $userId = $user->getId();
            $executed = $executed && $stmt->execute();

            if (!$executed) {
                throw new \Exception($conn->error);
            }
        }
        $stmt->close();
        
        return $this;
    }

    public function jsonSerialize() {
        return array(
            "id" => $this->getId(),
            "title" => $this->getTitle(),
            "source" => $this->getSourceEntity(),
            "number" => $this->getGrantNumber(),
            "originalAmount" => $this->getOriginalAmount(),
            "balance" => $this->getBalance(),
            "status" =>  $this->getStatus(),
            "administrator" => $this->getAdministrator(),
        );
    }

}
