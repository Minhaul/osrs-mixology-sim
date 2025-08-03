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
AAA was found 357160076 times
MMM was found 357152123 times
LLL was found 357141713 times
AAM was found 285675677 times
AAL was found 285722227 times
MMA was found 285714389 times
MML was found 285707329 times
LLA was found 285711335 times
LLM was found 285712562 times
MAL was found 214302569 times
Complete All Orders:
        input points:
        MixologyPoints { M: 29999906130, A: 30000043290, L: 30000050580 }

        output points:
        MixologyPoints { M: 39999974826, A: 40000055508, L: 40000322796 }

        output:input:
        M: 1.3333367, A: 1.3333333, L: 1.3333418

Complete All Orders Unless MAL, then only do MAL(s):
        input points:
        MixologyPoints { M: 26162500910, A: 26162125830, L: 26162482240 }

        output points:
        MixologyPoints { M: 33724460690, A: 33723990052, L: 33724593060 }

        output:input:
        M: 1.2890381, A: 1.2890385, L: 1.289044

Complete Best Order Unless MAL, then do MAL(s):
        input points:
        MixologyPoints { M: 9944650220, A: 7347295510, L: 13156475310 }

        output points:
        MixologyPoints { M: 12092030534, A: 9595968144, L: 15101437544 }

        output:input:
        M: 1.2159333, A: 1.3060545, L: 1.1478331

Complete Best Order Unless MAL, then do all:
        input points:
        MixologyPoints { M: 13782055440, A: 11185212970, L: 16994043650 }

        output points:
        MixologyPoints { M: 18367544670, A: 15872033600, L: 21377167280 }

        output:input:
        M: 1.3327144, A: 1.4190193, L: 1.2579211

Don't do MMM, AAA, or LLL:
        input points:
        MixologyPoints { M: 19639750560, A: 19335772170, L: 20247461550 }

        output points:
        MixologyPoints { M: 27174628750, A: 26971953956, L: 27580086544 }

        output:input:
        M: 1.3836545, A: 1.3949251, L: 1.3621503

Do Best Order Unless MAL, then do all but MMM, AAA, or LLL:
        input points:
        MixologyPoints { M: 12306091740, A: 9708893950, L: 15518052590 }

        output points:
        MixologyPoints { M: 16202741690, A: 13706877564, L: 19212309728 }

        output:input:
        M: 1.316644, A: 1.4117857, L: 1.238062

Don't do MMM, AAA, or LLL, unless MAL exists then do all:
        input points:
        MixologyPoints { M: 21115714260, A: 20812091190, L: 21723452610 }

        output points:
        MixologyPoints { M: 29339431730, A: 29137109992, L: 29744944096 }

        output:input:
        M: 1.3894596, A: 1.4000087, L: 1.3692548
```
