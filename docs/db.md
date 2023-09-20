# database notes

## join operations

### natural join

只考虑两个表中名字一样的字段，其他的字段不考虑
但是可能会出现的副作用是判断更多的字段，导致筛选出的数据更少
结局方案：采用 **join + using**

~~~sql
select name, title
from (instructor natural join teachers) join course using (course_id)
~~~

## string operations

### 通配符

- ~%~ matches any substring
- ~_~ matches any single character
  
## ordering operations

- order by: 按照某个字段排序
- group by: 按照某个字段分组
- having: 对分组后的数据进行筛选
- asc: 升序
- desc: 降序
- between xxx and xxx: 在某个范围内

## set operations

- union: 并集
- intersect: 交集
- except: 差集

~~~sql
(select course_id from section where semester = 'Fall' and year = 2009)
union
(select course_id from section where semester = 'Spring' and year = 2010)
~~~

union will **remove duplicate rows,** while union all will not

## aggregate functions

- count
- sum
- avg
- max
- min

~~~sql
select count(*) from student;
~~~

### distict clause

use distict to remove duplicate rows

~~~sql
select distinct course_id from section;
~~~

### group by

~~~sql
select course_id, count(*) as num_sections
from section
group by course_id;
~~~

每个course_id作为一个分组，然后对每个分组进行count操作

### having clause

对结果进行筛选

~~~sql
select course_id, count(*) as num_sections
from section
group by course_id
having count(*) >= 2;
~~~

### 优先级

from > where > group by > having > select > order by

## nested queries

### nested queries in where clause

~~~sql
select course_id, sec_id, semester, year
from section
where course_id in 
    (select course_id from section where semester = 'Fall' and year = 2009)
~~~

### set comparison operators

- all
- some

~~~sql
select distict name from student
where gpa > all/some
    (select avg(gpa) from student where major = 'CS');
~~~

- exist
  exist returns true if the nested query returns at least one row

~~~sql
select course_id from section as S
where semester = 'Fall' and year = 2009
and exist(
    select * from section as T
    where semester = 'Spring' and year = 2010
    and S.course_id = T.course_id
)


## with clause

~~~sql
with temp (course_id, sec_id, semester, year) as
    (select course_id, sec_id, semester, year
    from section
    where semester = 'Fall' and year = 2009)
select course_id, sec_id, semester, year
from temp
~~~
