extern crate ncurses;
use ncurses::*;

pub fn print() {
    initscr();

    let aa = "    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWWXkWWkKpWpppppppppWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWgNMMMMMMMMMMMMNgggkWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWQMMMMMMMMMMMMMMMMMMMMMNNWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWQNMMMMMMMMMMMMMMMMMMMMMMMMMMNKpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWWWNMMMMMMMMMMMMMMMMMMMMMMMMMMMNMMMkppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWQMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppWWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppWdMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNNMMMMMMMMMNNNggWWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppbWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNNkWWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#''''''''?7''7!?7''TMMMMMMMMMMMMNmkWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppWdMMHMMMMMMMMMMMMMMMMMMMMNMMMMMMMMMMD` ..J-.....             ~?''TMMMMMMMMNgKpppbWNMMMMMMMMMNNgWWWpppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppbWdMMNMMMMMMMMMMMMMMMMMMMMMMMM#''!` ..gNMMMMMMMMMMMN,                 ?''MMMMMMMNmWNMMMMMMMMMMMMMMMMMNKWppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppKMMMMMMMMMMMMMMMMMMMMMMMMB7     .MMMMMMMMMM#dMMMMMMNJ,                  .?MMMMMMNMMMMMMMMMMMMMMMMMMMNgWppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppMMMMMMMMMMMMMMNMMMMMM#^       .MMMMMMMMMMMMMMMMMMMMMMp                     TMMMMMMMMMMMMMMMMMMMMMMMMMMNWWpppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppdMMMMMMMMMMMMMMMMMMD!      .gMMMMMMMMMMMMMMMMMMMMMMMM#                       ?MMMMMMMMMMMMMMMMMMMMMMMMMMMKWpppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppMMMMMMMMMMMMMMMMM''`     .dMMMMMMMMMMMMMMMMMMMMMMMMMMMF   `  `  `               .TMMMMMMMMMMMMMMMMMMMMMMMMMNWppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppMMMMMMMMMMMMMM@!       .MMMMMMMMMMMMMMMMMMMMMMMMMMMMMF            `               MMMMMMMMMMMMMMMMMMMMMMMMMNkpppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppbpMMMMMMMMMMMM^      .JMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM]               `             TMMMMMMMMMMMMMMMMMMMMMMMMMRWpppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppdMMMMMNMMMM'     .dMMMMMM8dMMMMMMMMMMMMMMMMMMMMMMMMMM!                     .(ggggg,  TMMMMMMMMMMMMMMMMMMMMMMMM#pppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppdMMMMMMMMD      .dMMNNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM@    ...((,  `    ..MMMMMMMMMMMN, dMMMMMMMMMMMMMMMMMMMMNMMMWppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMM^       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM@   dMMMMMMMN,   .MMMMMMMMMMMMMMMN,?MMMMMMMMMMMMMMMMMMMMMMMNWpppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMM%        MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM''''!  .MMMMMMMMMN, `dMMMMMMMMMMMMMMMMM|.MMMMMMMMMMMMMMMMMMMMNMMHpppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMF          dMMMMMMMMMMMMMMMMMMMMMMMMMMM#''!        ,MMMMNMMMMM] .MMMMMMMNMMMMMMMMMMN,MMMMMMMMMMMMMMMMMMMMMWMNpppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppbWMMMMF,          ,WMMMMMMMMMNMMMMMMMMMMMMM#B=           ,MMMMMMMMMF  ,MMMMMMMMMMMMMMMMMMM)MMMMMMMMMMMMMMMMMMMMMMMNWppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppQMMMMF!`                 .7''''''''MMMW''''''!                   JMMMMMMMD   ,MMMMMMMMMMMMMMMMMMMRJMMMMMMMMMMMMMMMMMMMM#MMWpbppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMD        `                                           .MMMMMMM'    ,MMMMMMMMMMMMMMMMMMMN TMMMMMMMMMMMMMMMMMMM#MMNppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppWMMMM^            `                                       .MMMM''!        MMMMMMMMMMMMMMMMMMMN  dMMMMMMMMMMMMMMMMMMNMMXppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppWdMMMF                                                `   .MMMM$           MMMMMMMMMMMMMMMMMMF   HMMMMMMMMMMMMMMMM#dM#pppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppWMMM#        `        `                     `      `      ,MMMF            ,HMMMMMMMMMMMMMMMMF    UMMMMMMMMMMMMMMNMMMWpppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppbWMMMM/           `  `     `   `     `    `      `          (MMM`             .MMMMMMMMMMMMMMMMF     TMMMMMMMMMMMNMMMMXppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppWMMMF                                                     .MMMF                TMMMMMMMMMMMMMM3     .MMMMMMMMMMMMMMNppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppdMMM`                                                     JMMM:                 ?MMMMMMMMMMMM^      .WMMMMMMMMMMMWppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppbWMMMF          `                                           dMM#                    MMMMMMMMMM]        .MMMMMMMMNWppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppdMMM^                                          `          .MMMF                     ?MMMMMMMM!         ?MMMMMMWppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppMMMN                        `   `                         (MMM}                      ,MMNNM@'          .MMM#pppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppbpMMMF                                         `            dMM#                        ?''HM#'            HMMNpppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppMMMF                                                      MMMF                                          (MMNpppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppMMMF           `                                         .MMMb                                           MMNWppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppbbMMMF                                         `       .gMMMMMMN.                                          MMMWppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppMMMb                        `                      ..MMMMMMMMMMMN,                                       vMMWppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppdMMM,      `                    `                `.MMMMM''   TWMMMM,                                      ,MMkppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppWMMM@                                        `   .MMMM@!      (MMMb                                      .M#Wppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppWMMM,         `                                .MMMM^         dMMM,                                     .gWpppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppbppWMMMN,                     `                  .MMMM^          ,MMMb                                    .MMWpppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppWMMMb                                        JMMMF            ,MMM{                                  .MM#ppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppWMMM]                                      .MMMD             ,MMM]                               `.gMMMWppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppMMMMx      `                `             MMMM`              MMMN                               .MMMMNpppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppWMMMN,                  `                .MMMMN,             MMMM~                             .MMMHpppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppWMMMMN,                               ` JMMMMMt             MMMF                             .MMM#ppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMF                                 TMMM#'             dMMN                           .(MMMMMWppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppWMMMb   `                              .dNm..         ..MMMMF                          .MMMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppdMMN,              `               .MMMMMMMMMMMMMMMMMMMMMM!                       `.MMMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMN,                 `            TMMMMMMMMMMMMMMMMM#''`                         (MMMMMHpppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMe..                                     ?''''''?!                          .JgMMMMMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppdMMMMNb..        `                                                      `..MMMMNNMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMML.                  `                                   `  ...gMMMMMMMMMMNpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppXMMNMMMMMNgMMMMN[ ..  .....Jggggga...   ....gNNMMMMMM, ....JJgNMMMMMMMMMMM#=.MMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMNMMMMMMMHQMMMMMMMMMMMMMMMMMMMMMMMMMMMM''WMMMMMMMMMMMMMMMMMMMMMMMMMY''''''   .MMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppdMMM''77777WNgdMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM''''''''^!            MMMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMM]               ``                     ```??'                          MMMMNWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMM#                                                                      MMMMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMF                                                                     .MMMMppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppbpWMMMN                                                                     -MMM#ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMM.                                                   ..               JMMMHppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMM.                 .(Nm,                          .JMMMMNx            MMMMHppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppbpMMMM}              .JMMMMM#`                     ..gMMMMMMMMM/          .MMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMM`          ..(MMMMMMMM$     `                MMMMMMMMMMMM^          dMMMHpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMM,        .JMMMMMMMMMMF                      JMMMMMMMMMMF           .MMM#Wpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMM@      MMMMMMMMMMMMMN                      .MMMMMMMMMM^            dMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppbppMMM@      .TMMMMMMMMMMMF                       MMMMMMMM@             .MMMMpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMM#        -MMMMMMMMMM[                       (MMMMMMF             .MMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMN         .HMMMMMMMMb                    `  dMMMMMF              dMMM#ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMM_          dMMMMMMM:       `               ,MMMMM`             .MMMMXppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMM]           MMMMMM#                         MMMMt              .MMMNpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMF           ,MMMMMM$                        TMMD              .dMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMM#            .MMMMM|                                          ,MMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWdMMMN.            ,MMM#        `            `                     MMMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppNMMMMMM,             .''''^                                          .MMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMb      `                                                    MMMMHpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMN                       `                                  .MMM#Wpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMN,                                     `                   MMMMHppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMb                                                        (MMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMM.                                                      .MMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMM]    `   `              `                `             .MMM#ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMMN                                    `                .MMMMHppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppXHMMMMMMMMM]                    `                             ` (MMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMM@                                           `   .NMMMMMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppbpppdMMM,. ..                                      .gNMMMMMMMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMb                      `  ` .....+NMMMMMMMMMMMMMMM#ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppdMMMMMMMM]            `       ...(MMMMMMMMMMMMMMMMMMMMMMMMHppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMb    `   ...+gMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM#pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMb.(ggMMMMMMMMMMMMMMMMMMMMMHWpbpppWMMMMMMMMMMMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMMMMMMMMMMMMMMHHWppppppppppppppWMMNMMMMMMMMMNppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMMMMMMMHWWppbpbppppppppppppppppppXMMMMMMMMMMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMMkpbpppppppppppppppppppppppppppppMMMMMMMMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMMMMNpppppppppppppppppppppppppppppppdMMMMMMMM#ppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMMMMMXpppppppppppppppppppppppppppppppMMMMMMMMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppMMMMMMHpppppppppppppppppppppppppppppbpWMMMMMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMMM#pppppppppppppppppppppppppppppppWMMMMMMWppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppbpbdMMMM#ppppppppppppppppppppppppppppppppMMMMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMMM#pppppppppppppppppppppppppppppbppWMMNpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppWMMM#bppppppppppppppppppppppppppppppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp
    ppbpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppdMMWpppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppb";

    addstr(aa);
    refresh();
    getch();
    endwin();
}
