<?php

namespace CS450\Controller;

/**
 * @codeCoverageIgnore
 */
class GrantController
{
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

    /**
     * @Inject
     * @var CS450\Model\GrantFactory
     */
    private $grantFactory;

    public function __invoke($params)
    {
        if (empty($params["token"]) || $params["token"]["role"] !== "ADMINISTRATOR") {
            return;
        }

        return $this->grantFactory->findGrantsForUserId($params["token"]["uid"]);
    }

    public function grantsForAdmin($params)
    {
        if (empty($params["token"]) || $params["token"]["role"] !== "ADMINISTRATOR") {
            return [];
        }

        return $this->grantFactory->findGrantsAdministeredBy($params["token"]["uid"]);
    }

    public function updateGrantStatus($params)
    {
        if (empty($params["token"]) || $params["token"]["role"] !== "ADMINISTRATOR") {
            throw new \Exception("You are not authorized to approve or deny grants. Please talk to your administrator");
        }

        $newStatus = $params["post"]["status"];

        $statusMap = array(
            "APPROVE" => "APPROVED",
            "DENY" => "DENIED",
            "PENDING" => "PENDING",
        );

        if (!array_key_exists($newStatus, $statusMap)) {
            throw new \Exception("$newStatus is an unknown status. Please use a known status");
        }

        $newStatus = $statusMap[$newStatus];
        $grantId = $params["params"]["id"];

        $updateGrantStatusQ = <<<EOD
            UPDATE tbl_fact_grants
            SET status='$newStatus'
            WHERE id=$grantId
        EOD;

        $result = $this->db->getConnection()->query($updateGrantStatusQ);
        if (!$result) {
            $errMsg = sprintf("An error occurred executing your query: %s, %s", $updateGrantStatusQ, $conn->error);
            throw new \Exception($errMsg);
        }
    }
}
