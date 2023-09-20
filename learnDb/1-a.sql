with stu(ID, course_id) as (
    select `ID`, course_id from takes
    where takes.year < 2009 or (takes.year = 2009 and takes.semester in ('Fall', 'Winter'))
)
select distinct stu.ID, S.name
from student as S, stu
where stu.ID = S.ID;