<?php

namespace CS450\Service\Db;

use CS450\Service\DbService;

class MysqlDb implements DbService {

    /**
     * 
     * @Inject("db")
     */
    private $db;

    /**
     * 
     * @Inject
     * @var \Psr\Log\LoggerInterface
     */
    private $logger;

    private $conn;

    public function getConnection() {
        $this->logger->debug(sprintf(
            "Returning mysql connection to %s@%s/%s",
            $this->db['user'],
            $this->db['host'],
            $this->db['name'],
        ));
        
        $this->conn = new \mysqli(
            $this->db['host'],
            $this->db['user'],
            $this->db['pass'],
            $this->db['name'],
        );

        return $this->conn;
    }

    public function __destruct() {
        if(!empty($this->conn)) {
            $this->conn->close();
        }
    }
}
