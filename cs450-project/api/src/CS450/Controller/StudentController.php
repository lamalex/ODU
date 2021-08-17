<?php 

namespace CS450\Controller;

class StudentController {
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
        $sql = "SELECT uin, name FROM tbl_fact_students;";

        $conn = $this->db->getConnection();
        $result = $conn->query($sql);

        $this->logger->info(sprintf("Fetched %d rows", $result->num_rows));

        if($conn->error) {
            $this->logger->error($conn->error);
            throw new \Exception($conn->error);
        }

        $students = $result->fetch_all(MYSQLI_ASSOC);

        return $students;
    }
}
