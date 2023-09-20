with grade_info(ID, grade, row_num) as (
    select `ID`, grade, row_number() over (
        partition by `ID`, course_id
        order by
        year ,
        CASE semester
            when 'Fall' then 1
            when 'Winter' then 2
            when 'Spring' then 3
            when 'Winter' then 4
        end
        ) as row_num
    from takes
)

select sum(get_grade(grade)) as tot_cred from grade_info where grade_info.row_num = 1;

create function get_grade(grade_ varchar(2)) returns decimal(4,2)
deterministic
no sql
begin
    declare res decimal(4,2);
    case grade_
        when 'A+' then set res = 4.3;
        when 'A' then set res = 4.0;
        when 'A-' then set res = 3.7;
        when 'B+' then set res = 3.5;
        when 'B' then set res = 3.2;
        when 'B-' then set res = 3.0;
        when 'C+' then set res = 2.7;
        when 'C' then set res = 2.5;
        when 'C-' then set res = 2.3;
        else set res = 0.0;
    end case;
    return res;
end;