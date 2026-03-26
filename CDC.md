# GENESER 

- Le generateur de architecture.

## Description.

- Geneser CLI est un generateur d'architecture , c'est un programme en CLI , qui permetra au utilisateur de pouvoir initialiser un projets le plus vite possible avec des templates d'architectures celebres et qui sont aussi super.
- Nous avons des architectures de beacoup de personne celebres notament dans le cours de Andrea Bizzoto (Code With Andrea ) , CodeWithAndrea propose une architecture que moi j'utilise sur presque tous mes projets (Riverpod , GoRouter , Drif ,etc ) , mais le probleme c'est que ca met beacoup de temps a initialiser manuellement , creer les dossiers fichiers , etc , je ne parle meme pas encore la des nested_navigations et pourtant ce sont des taches tres repetitif , mon but est de mettre en place un CLI qui initialise c'est projets la pour moi du genre , je fait 

```bash
geneser --template flutter

-- ca me donne ensuite des liste de templates .

-- ca me donne aussi le chois de creer mon templates moi meme.

-- -- la elle me fait choisir les state que je veut utiliser , les architectures de dossier que je veut utiliser , les gestions de route que je veut utiliser

```
- Pour la V1 , la fonctionnalite initialise projets with template CodeWithAndrea doit a tous prit marcher , et ainsi que le choix de personalisation la .



# FONCTIONNALITE : 

- Ajout de nouvelles architecture avec 
``bash
geneser add-template
```
- Choisir une templates avec :
```bash
geneser --template flutter
```

- un fois dans le choix de template on a 2 options , soit choisir des templates prexistant - soit faire des templates nous meme on rassemblons plusieur librairies que l'on aime , par exemple : **GO_ROUTER** , **FLUTTER_GEN** , **GetX**.
