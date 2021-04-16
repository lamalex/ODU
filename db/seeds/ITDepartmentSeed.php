<?php


use Phinx\Seed\AbstractSeed;

class ITDepartmentSeed extends AbstractSeed
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
        $sql = file_get_contents(__DIR__ . '/../sql/011_add_it_department.sql');
        $this->execute($sql);
    }
}
