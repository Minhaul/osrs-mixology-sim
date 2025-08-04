# osrs-mixology-sim
Simple script simulating different strategies for playing the Mastering Mixology minigame in Old School RuneScape. I made this because I wanted to know roughly what the optimal strategy was for maximizing the amount of points earned per paste input into the minigame. This script assumes a player with 81 herblore meaning they have access to all potions in the minigame. Information gained from this script can be found on the OSRS wiki [here](https://oldschool.runescape.wiki/w/Mastering_Mixology/Strategies#Prioritise_total_points).

## Findings
At a baseline, if you just blindly make every potion in every order, you'll be earning roughly 1.33x points per paste for all 3 of Mox, Aga, and Lye equally.

Whenever a MAL shows up in your order, it's always worth making all 3 potions in the order for the bonus 1.4x points. This ends up always increasing the total ratio of points:paste.

The best strategy that I found was to never do the single paste potions (MMM, AAA, or LLL) UNLESS there was an MAL potion in the order. When there was an MAL it was worth doing all potions including single paste potions. This resulted in the highest overall points:paste across all three of Mox, Aga, and Lye.

Note that I came up with these strategies manually and just wrote functions to test those human invented strategies. This isn't computationally generating strategies to find the overall most optimal one. There could be a slightly more optimal strategy than what I've come up. I do think that what I've come up with is a good strategy and it was based on the knowledge I gained from testing all strategies leading up to this one.

Also note that this doesn't account for any use of digweed. Using digweed only in MAL potions will only increase the points:paste ratio.

## Raw Output
### The results of the script ran at 1,000,000,000 (1 billion) simulated mixology orders are:

```
AAA was found 357141122 times
MMM was found 357121837 times
LLL was found 357133391 times
AAM was found 285723563 times
AAL was found 285714934 times
MMA was found 285717003 times
MML was found 285720912 times
LLA was found 285686642 times
LLM was found 285740558 times
MAL was found 214300038 times
Complete All Orders:
        input points:
        MixologyPoints { M: 30000055000, A: 30000040430, L: 29999904570 }

        output points:
        MixologyPoints { M: 40000571814, A: 40000281426, L: 40000199456 }

        output:input:
        M: 1.3333501, A: 1.333341, L: 1.3333442

Complete All Orders Unless MAL, then only do MAL(s):
        input points:
        MixologyPoints { M: 26162622700, A: 26161998220, L: 26161919200 }

        output points:
        MixologyPoints { M: 33724851262, A: 33723913318, L: 33723859516 }

        output:input:
        M: 1.2890471, A: 1.289042, L: 1.2890438

Complete Best Order Unless MAL, then do MAL(s):
        input points:
        MixologyPoints { M: 9944954050, A: 7346994080, L: 13156232670 }

        output points:
        MixologyPoints { M: 12092276270, A: 9595565280, L: 15101118560 }

        output:input:
        M: 1.2159208, A: 1.3060532, L: 1.14783

Complete Best Order Unless MAL, then do all:
        input points:
        MixologyPoints { M: 13782386350, A: 11185036290, L: 16994218040 }

        output points:
        MixologyPoints { M: 18367996822, A: 15871933388, L: 21377458500 }

        output:input:
        M: 1.3327152, A: 1.4190328, L: 1.2579254

Don't do MMM, AAA, or LLL:
        input points:
        MixologyPoints { M: 19640748010, A: 19336407950, L: 20247559950 }

        output points:
        MixologyPoints { M: 27176246120, A: 26973018286, L: 27580459414 }

        output:input:
        M: 1.3836665, A: 1.3949343, L: 1.3621621

Do Best Order Unless MAL, then do all but MMM, AAA, or LLL:
        input points:
        MixologyPoints { M: 12306714010, A: 9708921840, L: 15518213750 }

        output points:
        MixologyPoints { M: 16203561200, A: 13707089084, L: 19212686322 }

        output:input:
        M: 1.3166441, A: 1.4118034, L: 1.2380732

Don't do MMM, AAA, or LLL, unless MAL exists then do all:
        input points:
        MixologyPoints { M: 21116420350, A: 20812522400, L: 21723564240 }

        output points:
        MixologyPoints { M: 29340681742, A: 29137862590, L: 29745231592 }

        output:input:
        M: 1.3894724, A: 1.400016, L: 1.3692611

Complete at most 1 single per order, unless MAL exists then do all:
        input points:
        MixologyPoints { M: 26801326630, A: 25039834040, L: 28865983620 }

        output points:
        MixologyPoints { M: 36191733150, A: 34822760772, L: 37762250772 }

        output:input:
        M: 1.3503709, A: 1.3906946, L: 1.3081921

Complete at most 1 single per order unless multiple singles, unless MAL exists then do all:
        input points:
        MixologyPoints { M: 23302945960, A: 21541611050, L: 25367592870 }

        output points:
        MixologyPoints { M: 31527231502, A: 30158422732, L: 33097764580 }

        output:input:
        M: 1.352929, A: 1.4000077, L: 1.3047262
```
