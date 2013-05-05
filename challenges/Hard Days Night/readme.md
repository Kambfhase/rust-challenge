# Hard Day's Night

It's been a hard day and the four friends Goerge, Paul, John and Ringo want to hang out together. They could either meet at a friends place or go to a local pub. After a long argument, they decide to meet at the place with the least total travel cost. The travel cost is just the distance from one place to another.

The input starts with the number of testcases, then follows the number `N` of friends. After that follow `N` lines, each with the name of a friend. Then one line with `M` the number of hangout places. Again, `M` lines with names for the hangout places. Now comes `E`, the number of given edges. Each edge has two names (the end points) and an integer, the actual distance, each on its own line. If you want to, you can also parse the JSON file as input.

Output a single line per testcase, containing the name of the point the friends will meet at.

## Example Input

    2
    4
    Paul
    Ringo
    John
    George
    2
    Irish Pub
    Golden Moose
    8
    Paul
    Ringo
    14
    Irish Pub
    Paul
    3
    Paul
    John
    3
    Irish Pub
    John
    5
    George
    John
    2
    George
    Golden Moose
    5
    John
    Golden Moose
    6
    Ringo
    Golden Moose
    1
    3
    Justus Jonas
    Bob Andrews
    Peter Shaw
    0
    3
    Justus Jonas
    Bob Andrews
    21
    Justus Jonas
    Peter Shaw
    15
    Bob Andrews
    Peter Shaw
    39

## Example Output

    John
    Justus Jonas

## Hint

The test cases are rather nice: The friends always have a place to meet. There is always one unique solution. The distances are integers greater than 0. The input data contains atmost one connection between two places. All connections are bi-directional. There are atleast two friends.
