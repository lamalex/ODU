<?php


use Phinx\Seed\AbstractSeed;

class DepartmentSeeder extends AbstractSeed
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
        $this->execute('SET FOREIGN_KEY_CHECKS = 0');
        $this->execute('TRUNCATE TABLE tbl_fact_departments');

        $sql = file_get_contents(__DIR__ . '/../sql/011_add_it_department.sql');
        $this->execute($sql);
        $sql = file_get_contents(__DIR__ . '/../sql/012_add_departments.sql');
        $this->execute($sql);
    }
}
