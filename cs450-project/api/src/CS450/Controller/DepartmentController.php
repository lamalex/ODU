<?php 

namespace CS450\Controller;

class DepartmentController {
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

    public function __invoke() {
        $sql = "SELECT id, name FROM tbl_fact_departments;";

        $conn = $this->db->getConnection();
        $result = $conn->query($sql);

        $this->logger->info(sprintf("Fetched %d rows", $result->num_rows));

        if($conn->error) {
            $this->logger->error($conn->error);
            throw new \Exception($conn->error);
        }

        $departments = $result->fetch_all(MYSQLI_ASSOC);

        return $departments;
    }
}
