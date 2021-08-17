<?php
 
namespace CS450\Service;

interface DbService {
    /**
     * Returns a connection to the data store
     * This is a bad abstraction and needs to grow.
     * Returning the connection maintains coupling between
     * db and class since you'll just be calling mysql methods
     * 
     * Leaving it because i don't want to write an entire ORM
     * for this project, but there's a lot of boiler plate in doing
     * sql so we should be able to wrap a lot -- but for those times
     * when you can't.
     * 
     * Use with caution
     * 
     * @return string database connection
     */
    public function getConnection();
}
