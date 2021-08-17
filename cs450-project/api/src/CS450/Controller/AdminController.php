<?php

namespace CS450\Controller;

class AdminController
{
    /**
     * @codeCoverageIgnore
     */

     /**
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;
     
    /**
     * @Inject
     * @var CS450\Model\UserFactory
     */
    private $userFactory;

    /**
     * @Inject
     * @var CS450\Service\DbService
     */
    private $db;

    public function getFaculty($params) {
        if (empty($params["token"]) || $params["token"]["role"] !== "ADMINISTRATOR") {
            throw new \Exception("You are not authorized to list faculty. Please talk to your administrator");
        }

        return (object) $this->userFactory->getFacultyInDepartmentForAdminId($params["token"]["uid"]);
    }

    public function deleteFaculty($params) {
        if (empty($params["token"]) || $params["token"]["role"] !== "ADMINISTRATOR") {
            throw new \Exception("You are not authorized to delete faculty. Please talk to your administrator");
        }

        $userId = $params["params"]["id"];

        $deleteUserQ = <<<EOD
            UPDATE tbl_fact_users
            SET deleted=TRUE
            WHERE id=?
        EOD;

        $conn = $this->db->getConnection();
        $stmt = $conn->prepare($deleteUserQ);

        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s, %s", $selectEmailQ, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "i",
            $userId,
        ) && $stmt->execute();

        if (!$executed) {
            throw new \Exception($conn->error);
        }

        return $executed;
    }
}
