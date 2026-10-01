// DANFO CHRONICLES — Ink narrative script
// Get across Lagos before your sister's wedding starts.
// Compile with inklecate, or run directly via inkjs (web) / InkGD (Godot).

VAR time_left = 90
VAR cash = 2500
VAR vibe = 60

-> start

=== function clamp_stats() ===
{ cash < 0: ~ cash = 0 }
{ time_left < 0: ~ time_left = 0 }
{ vibe < 0: ~ vibe = 0 }
{ vibe > 100: ~ vibe = 100 }

=== start ===
OJOTA BUS STOP — 9:14am

Your sister's wedding starts by 11am for Lekki. Traffic don already tie for Third Mainland. A conductor dey shout "Lekki-Ajah, one chance!" from inside a rickety yellow danfo, while an okada man dey signal you from the other side.

CONDUCTOR: "Enter now now, we dey commot!"

+ [Jump inside the danfo] (Cheaper, but slow — Third Mainland go choke)
    ~ cash -= 300
    ~ time_left -= 25
    ~ clamp_stats()
    -> danfo_ride
+ [Take the okada instead] (Faster, riskier, costs more)
    ~ cash -= 800
    ~ time_left -= 10
    ~ clamp_stats()
    -> okada_ride
+ [Wait and check your phone for a Bolt] (Safer, but surge pricing dey dis hour)
    ~ time_left -= 8
    ~ clamp_stats()
    -> bolt_check

=== danfo_ride ===
INSIDE THE DANFO — Third Mainland Bridge

The bus dey packed tight, and one aunty beside you don already start to complain about "this country." Conductor dey hang for door, collecting fare with speed wey no be here. Bridge dey slow-slow crawl.

AUNTY: "My brother, na so dis country be. Endure am."

+ [Strike up conversation to pass time] (Small vibe boost)
    ~ vibe += 8
    ~ time_left -= 6
    ~ clamp_stats()
    -> conductor_trouble
+ [Put on headphones and zone out] (Ignore the go-slow stress)
    ~ vibe += 4
    ~ time_left -= 6
    ~ clamp_stats()
    -> conductor_trouble

=== okada_ride ===
OKADA — weaving through Ozumba Mbadiwe

Wind dey slap your face as the okada man dodge between danfos and potholes like professional. You dey enjoy the speed small, until una see a police checkpoint ahead — the rider slow down and start counting change quietly.

OKADA MAN: "Oga, no worry, na small something we go give them."

+ [Let him handle the checkpoint his way] (Costs a little, keeps moving)
    ~ cash -= 500
    ~ time_left -= 8
    ~ clamp_stats()
    -> lekki_arrival
+ [Insist on stopping properly and asking questions] (Slower, but you keep your money)
    ~ time_left -= 15
    ~ clamp_stats()
    -> checkpoint_delay

=== bolt_check ===
OJOTA BUS STOP — checking your phone

Bolt app dey show ₦4,200 for a ride to Lekki because of surge. Your data dey also finish small small as the app struggle to load. A danfo conductor still dey shout beside you, and an okada man just pull up asking if you need a ride.

OKADA MAN: "Oga you go still dey here when I don reach Lekki o."

+ [Book the Bolt anyway] (Expensive but stress-free)
    ~ cash -= 4200
    ~ time_left -= 15
    ~ clamp_stats()
    -> lekki_arrival
+ [Forget it, jump on the okada] (Cheaper and faster)
    ~ time_left -= 4
    ~ clamp_stats()
    -> okada_ride

=== conductor_trouble ===
THIRD MAINLAND BRIDGE — still crawling

The conductor dey argue with a passenger about change — "I don give you your balance na, abeg no dull me today." The whole bus dey drag am into the matter. Meanwhile, traffic just start moving small.

YOU (thinking): "If I miss this wedding because of ₦50 change wahala, e go pain me."

+ [Stay quiet, focus on the road ahead] (Keep your calm)
    ~ vibe += 5
    ~ time_left -= 10
    ~ clamp_stats()
    -> lekki_arrival
+ [Help settle the change dispute yourself] (Small cash loss, but bus moves faster)
    ~ cash -= 100
    ~ vibe += 10
    ~ time_left -= 5
    ~ clamp_stats()
    -> lekki_arrival

=== checkpoint_delay ===
POLICE CHECKPOINT — Ozumba Mbadiwe

An officer leans in, checks your bag lazily, asks where you dey rush to. You explain say na wedding. He waves you through with a warning, but you don already lose precious minutes.

OFFICER: "Wedding? Ehen, go well. Tell them make dem no forget rice o."

+ [Continue to Lekki]
    ~ time_left -= 5
    ~ clamp_stats()
    -> lekki_arrival

=== lekki_arrival ===
LEKKI — approaching the wedding venue

You can hear highlife music and generator hum from a few streets away. Your outfit dey slightly rumpled, your phone dey buzz with messages from your mother, but you don almost reach.

MOTHER (text message): "Where are you?? They are about to start the dance entry o!!"

+ [See how it plays out]
    -> ending

=== ending ===
{ time_left <= 0:
    -> ending_missed
- else:
    { vibe >= 60:
        -> ending_smooth
    - else:
        { cash <= 500:
            -> ending_broke
        - else:
            -> ending_barely
        }
    }
}

=== ending_missed ===
YOU MISSED THE ENTRY DANCE

Time finish before you reach. You arrive just as they're cutting the cake — your mother is not amused, but at least you made it before the food finished.
-> END

=== ending_smooth ===
SMOOTH LANDING

You walk in cool, calm, and right on time — dancing your way straight into the reception like Lagos traffic never stood a chance.
-> END

=== ending_broke ===
BROKE BUT BLESSED

You made it on time, but your wallet is crying. Worth it though — you didn't miss your sister's big entrance.
-> END

=== ending_barely ===
YOU MADE IT — BARELY

Sweaty, slightly late, but present. Your sister spots you from across the room and just shakes her head, smiling.
-> END
