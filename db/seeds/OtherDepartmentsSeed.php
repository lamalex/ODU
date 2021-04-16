<?php


use Phinx\Seed\AbstractSeed;

class OtherDepartmentsSeed extends AbstractSeed
{
    /**
     * Run Method.
     *
     * Write your database seeder using this method.
     *
     * More information on writing seeders is available here:
     * https://book.cakephp.org/phinx/0/en/seeding.html
     */
    public function run()
    {
        $sql = file_get_contents(__DIR__ . '/../sql/012_add_departments.sql');
        $this->execute($sql);
    }
}
