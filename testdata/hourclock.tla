---------------------- MODULE HourClock ----------------------
EXTENDS Naturals
VARIABLE hr
Init == hr \in (1 .. 12)
Next == hr' = IF hr # 12 THEN hr + 1 ELSE 1
HC == Init /\ [][Next]_hr
THEOREM HC => []Init
==============================================================
