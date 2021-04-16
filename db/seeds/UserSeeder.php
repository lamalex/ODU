<?php


use Phinx\Seed\AbstractSeed;

class UserSeeder extends AbstractSeed
{
    public function getDependencies()
    {
        return ['DepartmentSeeder'];
    }

    public function run()
    {
        $this->execute('SET FOREIGN_KEY_CHECKS = 0');
        $this->execute('TRUNCATE TABLE tbl_fact_users');

        $sql = file_get_contents(__DIR__ . '/../sql/013_add_default_users.sql');
        $this->execute($sql);
    }
}
