# Guida utente di SolidInserts 1.1.0

[English](user-guide.md)

SolidInserts allarga l’imbocco di un foro esistente per ricavare la sede di un inserto filettato a caldo. Il taglio parte dal bordo selezionato e procede verso l’interno del pezzo. Per ogni sede, SolidCraft aggiunge alla cronologia uno schizzo e un taglio di estrusione modificabili.

## Installazione su SolidCraft 0.42.0

1. Scarica `solidinserts-1.1.0.zip` dai [rilasci di SolidInserts](https://github.com/NoOneExceptEnderk/SolidInserts/releases). Per compilare lo ZIP da sorgente, segui le [istruzioni di compilazione](../README.it.md#compila-e-verifica).
2. Estrai lo ZIP nella cartella dei plugin del tuo sistema operativo. Lo ZIP contiene già la cartella `solidinserts`. Mettila direttamente sotto `plugins`, in modo che `plugins/solidinserts/plugin.json` e `plugins/solidinserts/plugin.wasm` siano nella stessa cartella.

   | Sistema | Cartella dei plugin |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` oppure `~/.local/share/solidcraft/plugins/` se `XDG_DATA_HOME` non è impostata |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Avvia SolidCraft desktop 0.42.0 e apri un progetto. In **Preferenze → Plugin** verifica che SolidInserts sia presente e abilitato.

Lo ZIP del plugin funziona con l’app desktop standard. Per selezionare i fori dal pannello e vedere l’anteprima 3D in tempo reale, installa lo stesso ZIP e avvia una build di SolidCraft compilata localmente con la patch. Il kit di anteprima contiene la patch e le risorse necessarie. [Istruzioni di compilazione della build con patch](desktop-preview.md#istruzioni-in-italiano)

## Preparazione del modello

Chiudi lo schizzo o lo strumento attivo e porta il cursore alla fine della cronologia. Usa un foro cilindrico che sbocca direttamente su una faccia piana, senza smusso. Se il foro è passante, seleziona l’imbocco dal lato di inserimento. L’allargamento parte da quel bordo e lascia intatto un tratto del foro originale sotto la sede.

## Scelta della serie, della filettatura e della compensazione

**Tipo** imposta la serie di inserti CNC Kitchen. La tabella riporta le quote della sede M3 con compensazione zero.

| Serie | Filettature supportate | Sede M3 (diametro × profondità) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

**Filettatura** imposta la filettatura della vite. **Automatica (Ø nominale)** confronta il diametro di ciascun foro con il diametro nominale delle filettature entro ±0,05 mm. Un foro da Ø3 mm corrisponde a M3, uno da Ø4 mm a M4. Per i fori di gioco o da maschiare, seleziona a mano la filettatura della vite. Per esempio, per un foro di gioco Ø3,2 mm per una vite M3 scegli **M3 × 0,50**.

La serie e la compensazione valgono per tutti i fori selezionati. La modalità automatica può assegnare filettature diverse nello stesso gruppo, purché la serie le supporti tutte. Una filettatura scelta a mano vale per tutto il gruppo. Esegui il plugin più volte se servono serie, compensazioni o filettature manuali diverse. Puoi selezionare fino a 24 imbocchi per volta.

**Diametro ±** aggiunge una correzione al diametro della sede di catalogo, da −1 a +1 mm in passi di 0,01 mm. Parti da **0 mm** per usare la quota di catalogo. Per un inserto Standard M3, **+0,10 mm** porta il diametro a Ø4,10 mm. La profondità resta 6,7 mm perché è pari alla lunghezza dell’inserto più 1 mm. Per regolare l’accoppiamento, stampa un piccolo provino nello stesso materiale e con lo stesso orientamento del pezzo finale.

Nella finestra standard i campi si chiamano **Type**, **Screw thread** e **Diameter compensation**. La selezione automatica si chiama **Automatic (nominal Ø)**.

## Uso con SolidCraft standard

1. Nella vista 3D, fai clic sul bordo del primo foro. Tieni premuto **Ctrl** (**Cmd** su macOS) mentre fai clic sugli altri bordi per aggiungerli o rimuoverli dalla selezione.
2. Apri **Plugin → Insert Seats** e imposta serie, filettatura e compensazione.
3. Premi **Run**. Se SolidCraft te lo chiede, approva il pacchetto. La finestra standard usa la selezione della vista 3D e applica direttamente i tagli. Per cambiare la selezione prima di applicarli, premi **Annulla**.

## Uso con la build locale con patch

1. Apri **Modella → Plugin → Insert Seats** dalla ribbon o usa il menu **Plugin**. Premi **Seleziona** accanto a **Seleziona i fori**.
2. Fai clic sull’imbocco circolare di un foro. Un clic semplice sostituisce la selezione. **Ctrl+clic** aggiunge o rimuove gli imbocchi (su macOS funziona anche **Cmd+clic**).
3. Imposta **Tipo**, **Filettatura** e **Diametro ±**. L’anteprima si aggiorna quando cambi la selezione o i parametri. Leggi le quote **Ø diametro × profondità** e controlla che il materiale evidenziato sia dal lato giusto. Il riempimento segue il tema e l’impostazione grafica del materiale rimosso. Se disattivi il riempimento, restano visibili i contorni.
4. Premi **Fine selezione** per smettere di selezionare e mantenere l’anteprima. Premi di nuovo **Seleziona** per cambiare gli imbocchi. Durante la selezione, **Esc** svuota la selezione ed elimina l’anteprima lasciando il pannello aperto. **Annulla** chiude il pannello e scarta l’anteprima.
5. Attendi **Sedi pronte**, poi premi **Esegui** per applicare i tagli preparati. Se il pannello segnala un errore, correggi la selezione o i parametri e attendi l’aggiornamento dell’anteprima.

## Controllo e annullamento dei tagli

Dopo l’esecuzione, controlla gli schizzi e i tagli `SolidInserts · …` nella cronologia. Per cambiare il diametro della sede, modifica la quota dello schizzo. Per cambiare la profondità, modifica la distanza di estrusione. Un solo **Annulla** subito dopo l’esecuzione rimuove tutte le sedi appena create.

I provini si trovano nella cartella `examples/` dei sorgenti di SolidInserts e del kit di anteprima. Per provare un gruppo di due fori, apri [pilot-holes.sc](../examples/pilot-holes.sc), seleziona gli imbocchi da Ø3 mm e Ø4 mm sulla faccia superiore e usa **Standard**, **Automatica (Ø nominale)** e compensazione **0 mm**. Otterrai una sede M3 da **Ø4 × 6,7 mm** e una sede M4 da **Ø5,7 × 9,1 mm**. Apri [insert-seats.sc](../examples/insert-seats.sc) per vedere la geometria e la cronologia del provino finito.

## Problemi comuni

| Sintomo | Cosa controllare |
| --- | --- |
| Il comando del plugin non compare | Controlla **Preferenze → Plugin** e verifica che la cartella si chiami `solidinserts` e contenga entrambi i file del plugin. |
| Mancano **Seleziona i fori** o l’anteprima | Avvia il binario di SolidCraft compilato localmente per usare il pannello con patch. In SolidCraft standard, seleziona gli imbocchi nella vista 3D prima di aprire il plugin. |
| Un bordo viene ignorato o l’imbocco risulta inadatto | Ruota il modello per vedere il lato di inserimento e fai clic sul bordo circolare dove la parete cilindrica incontra la faccia piana. Scegli un imbocco senza smusso e, in SolidCraft standard, rimuovi dalla selezione eventuali elementi estranei. |
| **Esegui** è disattivato nel pannello con patch | Chiudi lo strumento o lo schizzo attivo, porta il cursore alla fine della cronologia e seleziona almeno un imbocco valido. Attendi **Sedi pronte**. Se hai modificato il modello, seleziona di nuovo gli imbocchi. |
| La misura automatica fallisce o una filettatura non è disponibile | Se il foro è di gioco o da maschiare, scegli a mano la filettatura della vite. Se la misura non è disponibile nella serie scelta, usa una serie e un inserto compatibili. |
| La sede è più stretta del foro o raggiunge un bordo, una cavità o un’altra sede | Controlla il diametro del foro originale e la compensazione. Se l’inserto scelto richiede più spazio, rivedi l’interasse dei fori o il materiale attorno alla sede. |
| La profondità è insufficiente | Controlla lo spessore del corpo ed eventuali cavità interne. Usa un inserto compatibile più corto o modifica il pezzo per lasciare materiale sufficiente sotto la sede. |
| Il foro ha già una sede creata con SolidInserts | Modifica lo schizzo e il taglio esistenti nella cronologia. |
