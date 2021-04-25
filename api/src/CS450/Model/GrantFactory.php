<?php

namespace CS450\Model;

use CS450\Model\Grant;

final class GrantFactory {
    /**
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;

    /**
     * @Inject
     * @var CS450\Service\DbService
     */
    private $db;

    public function findAll(): array {
        $selectAllGrantsQ = <<<EOD
            SELECT tbl_fact_grants.id, tbl_fact_grants.title, tbl_fact_grants.status, tbl_fact_grants.balance, 
                tbl_fact_grants.original_amt, tbl_fact_grants.grant_number,
                entity.id AS `_entity_id`, entity.name AS `_entity_name`, entity.type AS `_entity_type`,
                admin.id AS `_admin_id`, admin.email AS `_admin_email`, admin.name AS `_admin_name`, 
                admin.user_role AS`_admin_role`, admin.department AS `_admin_department`
            FROM tbl_fact_grants
            JOIN tbl_fact_granting_entity AS entity
            ON tbl_fact_grants.source_id = entity.id
            JOIN tbl_fact_users AS admin
            on tbl_fact_grants.administrator_id = admin.id;
        EOD;

        $conn = $this->db->getConnection();
        $result = $conn->query($selectAllGrantsQ);

        if (!$result) {
            $errMsg = sprintf("An error occurred executing your query: %s, %s", $selectAllGrantsQ, $conn->error);
            throw new \Exception($errMsg);
        }

        $grants = [];
        while($grant = $result->fetch_object("CS450\Model\Grant", [$this->db])) {
            $grants[] = $grant;
        }

        return $grants;
    }
}
