BladeMountainAwakening
----

You wake up alone atop a mountain, tip of smooth, shiny black stone. The last
thing you remember was opening a door into... you can't quite recall.

> Goto -> BladeMountainSummit

BladeMountainSummit
----

You're at the summit of Blade Mountain. There is a staircase carved into one side
leading to a path down the mountainside.

> 'Look at the areas surrounding the mountain.'
> Goto -> BladeMountainView

> 'Search the tip of the mountain.'
> Check Perception 9 10xp -> BladeMountainNote BladeMountainFailedSearch

> 'Head down the staircase.'
> 'You head down the stairs.'
> Goto -> BladeMountainUpperPath

BladeMountainView
----

You approach the edge of the black stone to find a sharp drop of fifty feet
and a view for miles around the mountain.

To the north, south, and west, swamp extends as far as the eye can see past
the sharp slopes of the mountain. A faint mist surrounds the swamps.

To the east, the slopes descend into grassland and reach the sea.

| SwitchCheck Perception

6. You think you can see a road, but at this distance, it's hard to say for sure.

10. You see a road, it must be a good twenty miles away.

15. You see a dirt road, likely twenty feet across, just over twenty miles to
the east.

20. You see a cobbled road, stone fading beneath dust, exactly twenty feet
across. It seems quite well-travelled, and it's fairly straight, going straight
through large boulders at some points. It is twenty three miles east and a few
hundred yards north.

> 'Back away from the edge of the cliff.'
> Goto -> BladeMountainSummit

BladeMountainNote
----

You search the tip of the mountain carefully, finding a small note not twenty
feet from where you awoke.

> 'Read the note'
> 'The note mentions the "Gerr Empire," and seems to suggest you have gone
> somewhere you should not have.'
> Goto -> BladeMountainNote

> 'Pocket the note'
> AddItem BladeMountainNoteFromHand

You find nothing else of interest.

> Goto -> BladeMountainSummit

BladeMountainFailedSearch
----

You find nothing.

> Goto -> BladeMountainSummit

BladeMountainUpperPath
----

> Marker? EncounteredStanAtSummit
> Goto -> BladeMountainUpperPathStanEncounter

> Goto -> BladeMountainUpperPathNoEncounter

BladeMountainUpperPathStanEncounter
----

> Mark EncounteredStanAtSummit

You encounter a man fifty paces down the mountain path, he appears to be cleaning
brush off the path.

| SwitchCheck Perception

9. You note a recurve bow and quiver on his back.

| SwitchCheck Perception

12. He doesn't seem to have noticed you.

> 'Approach the man.'
> Goto -> BladeMountainApproachStan

> 'Attack the man.'
> Combat {
> 1x BladeMountainStan
> } -> BladeMountainDefeatedStan BladeMountainDefeatedByStan

> 'Try to avoid the man, giving the path a wide berth.'
> Check Sneak 15 50xp -> BladeMountainAvoidedStan BladeMountainFailedToAvoidStan

> 'Head back up the stairs.'
> Goto -> BladeMountainSummit

BladeMountainApproach
----

The man looks you up and down. Now that you're close, you note two long scars
running across his face.

Stan: Hello there, traveler. I didn't see you come up, have you been up long?

> 'Yes, I've been up for a few days.'
> Check Deception 8 20xp -> BladeMountainStanThinksTraveler BladeMountainCaughtInLie

> 'No, I just came up this path a few moments ago.'
> Check Deception 10 40xp -> BladeMountainStanThinksTraveler BladeMountainCaughtInLie

> 'Maybe. I don't really know how I got here.'
> Goto -> BladeMountainStanTruth

> ?PassiveCheck Smell 15
> '*sniff* blackberries and... steel? But you aren't carrying any steel.'
> Goto -> BladeMountainStanSmell

BladeMountainUpperPathNoEncounter
----

? Marker KilledStanAtSummit

The path, thoroughly washed out, seems a though it has not been used in a very
long time.

? Marker !KilledStanAtSummit

You find yourself on the well-kept path to the summit of Blade Mountain.

> 'Head down the path.'
> Goto -> BladeMountainCabin

> 'Head up the stairs to the summit.'
> Goto -> BladeMountainSummit

BladeMountainCabin
----

? Marker MetStan

You reach Stan's cabin.

? Marker !MetStan

You reach a small but sturdy cabin along the path.

> Exit

