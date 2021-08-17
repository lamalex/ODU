<?php


use Phinx\Seed\AbstractSeed;

class StudentsSeeder extends AbstractSeed
{
    public function getDependencies()
    {
        return ['DepartmentSeeder'];
    }
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
        $this->execute('TRUNCATE TABLE tbl_fact_students');

        $sql = file_get_contents(__DIR__ . '/../sql/014_add_students.sql');
        $this->execute($sql);
    }
}
