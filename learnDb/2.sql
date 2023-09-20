with emp_man(emp_no, man_no) as (
    select E.emp_no, M.emp_no
    from dept_emp as E, dept_manager as M
    where E.dept_no = M.dept_no and E.emp_no <> M.emp_no
)
select T.emp_no, T.man_no as manager_no, e.salary as emp_salary, m.salary as manager_salary
from emp_man as T
join salaries as e on e.emp_no = T.emp_no
join salaries as m on m.emp_no = T.man_no
where e.salary > m.salary;