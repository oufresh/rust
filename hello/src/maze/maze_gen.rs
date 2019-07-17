/**
 * Genera il labirinto che è un insieme di stanze.
 * I possibili percorsi sono delle sequenze di direzioni in cui si può o non si può andare e 
 * portano ad altre stanze: i percosi quindi sono rappresentati da un albero che si allarga ogni volta
 * con tante foglie quante sono le direzioni aperte in ogni stanza. Solo una stanza sarà definita come fine (una delle ultime foglie) e una la aradicve è l'inizio
 * 
 * In prima istanza non facciamo percorsi chiusi quindi da una stanza/foglia si può solo andare avanti e indietro ma non 
 * spostarsi su altri rami. Questo potrebbe rapppresentare il caso di stanze adiacenti in cui si ritorna in un'altro rammo. Quindi ogni porta porta sempre
 * ad un livello successivo dell'albero.
 *
 * Se voglio fare dei percorsi chiusi allora bisogna associare alle direzioni un identificativo della stanza e quindi poi fare dei modelli per definire le stanze
 * adiacenti (labirinto qudrato ... )
 * 
 * Per iniziare sarà un albero il cui nodo è una stanza che ha da 2 a 4 porte/direzioni/rami dell'albero con altrettante tanze successive/foglie. Da 1 a 4 
 * perchè potremmo non sempre avere le porte sui 4 lati ma anche una sola di ingresso e una di uscita.
 */