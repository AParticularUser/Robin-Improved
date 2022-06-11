# smashline_robin_improved
What if Robin had actually been finished being coded.
--------------------------------------------------------------------------
Change Log:
- spells and Levin-sword charge isn't consumed until the attack actually comes out
- spells and Levin-sword have a pasive recharge:
	* pasive recharge is ~2x slower than the empty recharge
	* spells/Levin won't recharge if said spell/Levin is being used
	* if thunder is stored, it won't pasively recharge
- Robin starts match with Levin-sword (not sure why this isn't already a thing)
- Robin only uses Levin-sword aerials with smash inputs (this means the only way to use Levin N-air is with AB-smash enabled)
- up-tilt starts one frame earlier to actually match the animation: 6 -> 5
- down-tilt adjusted bkb and angle for combos: bkb:40, angle:30 -> bkb:50, angle:68
- dash-attack increased range by changing animation to extend arm further out (less shank more thrust)
- moved hit-box slightly forward on Bronze (normal) f-smash to better match animation: 8-14 -> 10-16
- fixed Levin up-smash lingering flare effect not being removed
- adjusted begining auto-cancel window on aerials to match Levin usage:
	* f-air: 4-27 -> 10-27
	* b-air: 0-32 -> 7-32
	* u-air: 5-27 -> 9-27
	* d-air: 2-48 -> 11-48
	* n-air: 4-34 -> 6-34
- Bronze (normal) up-air active frames lasts one frame longer to better match the animation: 10-14 -> 10-15
- fixed Levin n-air's second hit lingering flare effect not matching active frames 
- increased standing grab range (now you don't have to be kissing your opponent to grab them): 4.0-9.2 -> 3.0-11.0
- increased pummel damage: 1.5 -> 1.6
- pummel requires and consumes dark magic (creates more opportunities for decision making)
- split up-special into two parts:
	* up-special-1 doesn't put Robin into free-fall and can only be used once per air time unless hit
	* up-special-2 behaves normally and will put Robin into free-fall
	* if up-special fails due to lack of wind-magic, Robin is not put into free-fall
- down-special comes out faster and lasts longer: 15-18 -> 8-24
- incresed down-special total frames: 49 -> 54
- if down-special is successful:
	* the enemy gains the "curse" effect
	* Robin fully recharges spells and Levin-sword excluding Rizaia
	* Robin can use up-special-1 again if it was already used
	* Robin is no longer put into special-fall
- down-special revival time: 1800 -> 780 (30sec -> 13sec)
- if you hold side-taunt it depletes the current book and gives it as an item
- if you up-taunt it swaps the current sword (Levin <-> Bronze)
