<?php

namespace CS450\Model;

use CS450\Service\DbService;

final class Grant {
    private $db;

    private $id;
    private $title;
    private $status;
    private $adminId;
    private $balance;
    private $sourceId;
    private $grantNumber;
    private $originalAmount;
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
        return $this->grantNumber;
    }

    public function getStatus() {
        return $this->status;
    }

    public function getAdminId() {
        return $this->adminId;
    }

    public function getSourceId() {
        return $this->sourceId;
    }

    public function getBalance() {
        return $this->balance;
    }

    public function getOriginalAmount() {
        return $this->originalAmount;
    }

    public function getRecipients() {
        return $this->recipients;
    }

    function startupGrant() {
        $oduSourceId = $this->db->getConnection()
            ->query("SELECT id FROM tbl_fact_granting_entity WHERE name='ODU'")
            ->fetch_object()
            ->id;

        $this->status = "APPROVED";
        $this->title = "Starup Fund";
        $this->sourceId = $oduSourceId;
        $this->grantNumber = "ODU-STARTUP";

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
        $this->grantNumber = $grantNumber;
        return $this;
    }

    function setSourceId($sourceId): Self {
        $this->sourceId = $sourceId;
        return $this;
    }

    function setOriginalAmount($originalAmount): Self {
        $this->originalAmount = $originalAmount;
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
        $this->adminId = $adminId;
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
            $this->grantNumber,
            $this->title,
            $this->sourceId,
            $this->originalAmount,
            $this->balance,
            $this->status,
            $this->adminId,
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
}
