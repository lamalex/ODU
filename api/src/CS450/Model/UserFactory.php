<?php 

namespace CS450\Model;

use CS450\Model\User;
use CS450\Lib\EmailAddress;
use CS450\Model\Grant;

final class UserFactory {
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

    public function findByEmail(EmailAddress $email): ?User {
        $selectEmailQ = <<<EOD
            SELECT id, name, email, password, user_role, department
            FROM tbl_fact_users
            WHERE email=?
        EOD;

        $conn = $this->db->getConnection();
        $stmt = $conn->prepare($selectEmailQ);

        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s, %s", $selectEmailQ, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "s",
            $email,
        ) && $stmt->execute();

        if (!$executed) {
            throw new \Exception($conn->error);
        }

        $this->logger->info("Running sql " . $selectEmailQ . "(=" . $email .")");

        $result = $stmt->get_result();
        $userRow = $result->fetch_assoc();

        return $userRow
            ? (new User($this->db))
                ->setId($userRow["id"])
                ->setName($userRow["name"])
                ->setEmail($userRow["email"])
                ->setRole($userRow["user_role"])
                ->setPasswordHash($userRow["password"])
                ->setDepartment($userRow["department"])
            : null;
    }
    public function findGrants(Grant $grantNumber): array {
        $selectGrantQ = <<<EOD
            SELECT a.name as faculty_name, c.grant_number, c.title, d.name as department 
            FROM tbl_fact_users a
            JOIN (
            SELECT grant_i,user_id
            FROM tbl_map_grant_users
            )b
                ON a.id = b.user_id
            JOIN tbl_fact_grants c
                ON b.grant_id = c.source_id
            JOIN tbl_fact_departments d
                ON a.department = d.id
            WHERE ic.status in ('PENDING', 'APPROVED');
        EOD;

        $conn = $this->db->getConnection();
        $stmt = $conn->prepare($selectGrantQ);

        if (!$stmt) {
            $errMsg = sprintf("An error occurred preparing your query: %s, %s", $selectGrantQ, $conn->error);
            throw new \Exception($errMsg);
        }

        $executed = $stmt->bind_param(
            "s",
            $grantNumber,q  
        ) && $stmt->execute();

        if (!$executed) {
            throw new \Exception($conn->error);
        }

        $this->logger->info("Running sql " . $selectGrantQ . "(=" . $grantNumber .")");

        $result = $stmt->get_result();
        $grantUsers =[];
        while($userGrant = $result->fetch_assoc()){
            $grantUsers[] = $userGrant; 
        };
        return $grantUsers; 
    }

    public function getFacultyInDepartmentForAdminId(int $id) {
        $selectFacultyQ = <<<EOD
            SELECT u.id, u.name, u.user_role, d.name as department FROM tbl_fact_users u
            LEFT JOIN tbl_fact_departments d
            ON u.department=d.id
            WHERE department = (
                SELECT department
                FROM tbl_fact_users
                WHERE id=$id
            )
            AND u.id NOT IN ($id)
            AND u.deleted=FALSE
        EOD;

        $result = $this->db->getConnection()->query($selectFacultyQ);

        if (!$result) {
            $errMsg = sprintf("An error occurred executing your query: %s, %s", $selectFacultyQ, $conn->error);
            throw new \Exception($errMsg);
        }
        
        $users = [];
        while($user = $result->fetch_object("CS450\Model\User", [$this->db])) {
            $this->logger->info(print_r($user, true));
            $users[$user->getId()] = $user;
        }

        return $users;
    }
}
