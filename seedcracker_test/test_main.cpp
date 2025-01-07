#define CATCH_CONFIG_MAIN
#include "catch_amalgamated.hpp"

#include "rng.hpp"
#include "seedcracker.hpp"
#include <stdint.h>

using namespace Catch;

TEST_CASE("Java RNG works", "[rng]") {
    SECTION("setSeed") {
        int64_t seed = 3485113479660343165;
        setSeed(&seed, seed);
        REQUIRE(seed == 171779162663184);
        seed = 8933480875778337940;
        setSeed(&seed, seed);
        REQUIRE(seed == 28053626391289);
        seed = -3410331583413443761;
        setSeed(&seed, seed);
        REQUIRE(seed == 19222171501858);
        seed = -2277860632936632095;
        setSeed(&seed, seed);
        REQUIRE(seed == 116374070089356);
    }
    SECTION("nextInt") {
        int64_t seed;
        seed = -6236408045622025274;
        setSeed(&seed, seed);
        REQUIRE(nextInt(&seed, -677897384) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 320459357) == 319310048);
        REQUIRE(nextInt(&seed, -925562977) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 582715302) == 330356403);
        REQUIRE(nextInt(&seed, 880145017) == 192216090);
        seed = -3971974982040224698;
        setSeed(&seed, seed);
        REQUIRE(nextInt(&seed, -708333636) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, -181680635) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 493934574) == 115255286);
        REQUIRE(nextInt(&seed, 54979026) == 50137185);
        REQUIRE(nextInt(&seed, 1035754773) == 523717380);
        seed = 4099247437432620754;
        setSeed(&seed, seed);
        REQUIRE(nextInt(&seed, -1059212296) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 319219134) == 111995990);
        REQUIRE(nextInt(&seed, 615717016) == 411819899);
        REQUIRE(nextInt(&seed, 664065545) == 254982495);
        REQUIRE(nextInt(&seed, 145483748) == 104987534);
        seed = 8512154456609035914;
        setSeed(&seed, seed);
        REQUIRE(nextInt(&seed, -1067179834) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, -844565905) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, -978961948) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, -984275430) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 696435624) == 282239151);
        seed = 6850361211147904260;
        setSeed(&seed, seed);
        REQUIRE(nextInt(&seed, -560962460) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 188786512) == 144218465);
        REQUIRE(nextInt(&seed, -627187239) == std::numeric_limits<int32_t>::min());
        REQUIRE(nextInt(&seed, 1049079114) == 164369193);
        REQUIRE(nextInt(&seed, -1019885765) == std::numeric_limits<int32_t>::min());
    }
    SECTION("nextBool") {
        int64_t seed;
        seed = -6825715404660402630;
        setSeed(&seed, seed);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
        seed = -7947889110491812041;
        setSeed(&seed, seed);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == true);
        seed = -3074560066088689181;
        setSeed(&seed, seed);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        seed = 1676532073683669973;
        setSeed(&seed, seed);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        seed = 2247844291703577844;
        setSeed(&seed, seed);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
        REQUIRE(nextBool(&seed) == false);
        REQUIRE(nextBool(&seed) == true);
    }
    SECTION("nextFloatREQUIRE") {
        int64_t seed;
        seed = -422986445787753782;
        setSeed(&seed, seed);
        REQUIRE(nextFloat(&seed) == Approx(0.21843565));
        REQUIRE(nextFloat(&seed) == Approx(0.28452128));
        REQUIRE(nextFloat(&seed) == Approx(0.44051987));
        REQUIRE(nextFloat(&seed) == Approx(0.6725568));
        REQUIRE(nextFloat(&seed) == Approx(0.7393489));
        seed = -1728208026221225390;
        setSeed(&seed, seed);
        REQUIRE(nextFloat(&seed) == Approx(0.28069735));
        REQUIRE(nextFloat(&seed) == Approx(0.3679428));
        REQUIRE(nextFloat(&seed) == Approx(0.9654926));
        REQUIRE(nextFloat(&seed) == Approx(0.33747226));
        REQUIRE(nextFloat(&seed) == Approx(0.4925785));
        seed = -8251487243020756333;
        setSeed(&seed, seed);
        REQUIRE(nextFloat(&seed) == Approx(0.09536797));
        REQUIRE(nextFloat(&seed) == Approx(0.041228473));
        REQUIRE(nextFloat(&seed) == Approx(0.9446709));
        REQUIRE(nextFloat(&seed) == Approx(0.9657801));
        REQUIRE(nextFloat(&seed) == Approx(0.035991132));
        seed = 112523361195652593;
        setSeed(&seed, seed);
        REQUIRE(nextFloat(&seed) == Approx(0.20806319));
        REQUIRE(nextFloat(&seed) == Approx(0.6132273));
        REQUIRE(nextFloat(&seed) == Approx(0.67684305));
        REQUIRE(nextFloat(&seed) == Approx(0.67940986));
        REQUIRE(nextFloat(&seed) == Approx(0.44298184));
        seed = -2395321196266959707;
        setSeed(&seed, seed);
        REQUIRE(nextFloat(&seed) == Approx(0.8843721));
        REQUIRE(nextFloat(&seed) == Approx(0.97955996));
        REQUIRE(nextFloat(&seed) == Approx(0.7559995));
        REQUIRE(nextFloat(&seed) == Approx(0.7264743));
        REQUIRE(nextFloat(&seed) == Approx(0.42587078));
    }
    SECTION("nextLong") {
        int64_t seed;
        seed = -9153832561302416652;
        setSeed(&seed, seed);
        REQUIRE(nextLong(&seed) == 5863046693895182875);
        REQUIRE(nextLong(&seed) == -7623228528331805881);
        REQUIRE(nextLong(&seed) == 7481044444525761822);
        REQUIRE(nextLong(&seed) == -3171896890307744384);
        REQUIRE(nextLong(&seed) == -793044611336039371);
        seed = -203028209119400807;
        setSeed(&seed, seed);
        REQUIRE(nextLong(&seed) == 5856212886049763432);
        REQUIRE(nextLong(&seed) == 6289087029572382135);
        REQUIRE(nextLong(&seed) == 3462628023599140868);
        REQUIRE(nextLong(&seed) == 2489199377324065454);
        REQUIRE(nextLong(&seed) == 7169474616144910587);
        seed = -5223093179286753968;
        setSeed(&seed, seed);
        REQUIRE(nextLong(&seed) == 8049062271010800086);
        REQUIRE(nextLong(&seed) == 4827960048714042849);
        REQUIRE(nextLong(&seed) == -3968486191364740160);
        REQUIRE(nextLong(&seed) == 1035177240494386716);
        REQUIRE(nextLong(&seed) == -9210978161071642116);
        seed = -3065726349274649202;
        setSeed(&seed, seed);
        REQUIRE(nextLong(&seed) == 393786121843493426);
        REQUIRE(nextLong(&seed) == 6276401612898046275);
        REQUIRE(nextLong(&seed) == -1413354435044834328);
        REQUIRE(nextLong(&seed) == -7200768126532195858);
        REQUIRE(nextLong(&seed) == 7286680463800330059);
        seed = -765158612100474295;
        setSeed(&seed, seed);
        REQUIRE(nextLong(&seed) == 2298229889755854233);
        REQUIRE(nextLong(&seed) == -7871799515833060475);
        REQUIRE(nextLong(&seed) == -3175373550869654801);
        REQUIRE(nextLong(&seed) == -6325245358190952736);
        REQUIRE(nextLong(&seed) == 2515923585135733062);
    }
    SECTION("getPopulationSeed") {
        int64_t seed;
        seed = 1623024503030186110;
        REQUIRE(getPopulationSeed(seed, 8813070, 19620696) == 276708866249204);
        REQUIRE(getPopulationSeed(seed, 10631644, 17944910) == 82207099186384);
        REQUIRE(getPopulationSeed(seed, 8680417, -2211947) == 52052742490216);
        REQUIRE(getPopulationSeed(seed, -7948852, -21332830) == 163831676759108);
        REQUIRE(getPopulationSeed(seed, 12335895, -11909847) == 151511376819346);
        seed = 8976634243353281220;
        REQUIRE(getPopulationSeed(seed, -16888028, 10258895) == 149743964026883);
        REQUIRE(getPopulationSeed(seed, 5289228, 14890018) == 72978385315122);
        REQUIRE(getPopulationSeed(seed, 18369236, 104001) == 208686965105509);
        REQUIRE(getPopulationSeed(seed, -19206981, -16885622) == 60747464901637);
        REQUIRE(getPopulationSeed(seed, 3802506, -3174118) == 85380082731632);
        seed = 6972613697542631438;
        REQUIRE(getPopulationSeed(seed, 19451980, 19717001) == 215709711540749);
        REQUIRE(getPopulationSeed(seed, 9846824, 1430892) == 151994969036802);
        REQUIRE(getPopulationSeed(seed, 12245121, -14135324) == 215223240367685);
        REQUIRE(getPopulationSeed(seed, -9643781, 13517620) == 215357704404015);
        REQUIRE(getPopulationSeed(seed, 15655741, 16302291) == 249636060326022);
        seed = -64717746766552495;
        REQUIRE(getPopulationSeed(seed, 2740243, 1899490) == 130413670911044);
        REQUIRE(getPopulationSeed(seed, -8214147, -12478584) == 69615404979304);
        REQUIRE(getPopulationSeed(seed, -1286895, -4889865) == 265740562613851);
        REQUIRE(getPopulationSeed(seed, 4519653, -5541236) == 270226422630940);
        REQUIRE(getPopulationSeed(seed, 12780884, 10681218) == 222319640432763);
        seed = 7662337536138481453;
        REQUIRE(getPopulationSeed(seed, -10544675, -1740511) == 230880289381329);
        REQUIRE(getPopulationSeed(seed, -19152966, 11187829) == 274098607373610);
        REQUIRE(getPopulationSeed(seed, -2768312, -5868758) == 60300160355623);
        REQUIRE(getPopulationSeed(seed, -20502464, 16797556) == 212766799447785);
        REQUIRE(getPopulationSeed(seed, -20184626, -8581567) == 48987559978642);
    }
    SECTION("getDecoratorSeed") {
        int64_t seed;
        seed = -5722685173690564176;
        REQUIRE(getDecoratorSeed(seed, -11802949, -20424998, 30001) == 57497380696337);
        REQUIRE(getDecoratorSeed(seed, 14315429, 12010478, 30001) == 185917727096387);
        REQUIRE(getDecoratorSeed(seed, 7992747, -8213193, 30001) == 8710054481704);
        REQUIRE(getDecoratorSeed(seed, 2824118, -7744489, 30001) == 238047743234483);
        REQUIRE(getDecoratorSeed(seed, 15651473, 8160347, 30001) == 198544346382006);
        seed = 6419375638244123891;
        REQUIRE(getDecoratorSeed(seed, -8637418, 19491500, 30001) == 124462486331627);
        REQUIRE(getDecoratorSeed(seed, 10777036, 1734734, 30001) == 66926199812159);
        REQUIRE(getDecoratorSeed(seed, 18433736, -20706036, 30001) == 16384670146597);
        REQUIRE(getDecoratorSeed(seed, 19207060, -17174183, 30001) == 142669661468706);
        REQUIRE(getDecoratorSeed(seed, 1263312, 7091282, 30001) == 156887947955327);
        seed = 8525635622147715264;
        REQUIRE(getDecoratorSeed(seed, 3081075, 3845640, 30001) == 43123668257949);
        REQUIRE(getDecoratorSeed(seed, 16389156, 4623318, 30001) == 107431622875698);
        REQUIRE(getDecoratorSeed(seed, 12909587, 9557264, 30001) == 120886536086309);
        REQUIRE(getDecoratorSeed(seed, -17997345, 189507, 30001) == 209839608913572);
        REQUIRE(getDecoratorSeed(seed, 5538530, -7099043, 30001) == 155834808665339);
        seed = -9175746650515795159;
        REQUIRE(getDecoratorSeed(seed, 18693801, -6677442, 30001) == 187404318751370);
        REQUIRE(getDecoratorSeed(seed, -6573701, -14600668, 30001) == 113776636767134);
        REQUIRE(getDecoratorSeed(seed, -6102010, 19739913, 30001) == 5946600298980);
        REQUIRE(getDecoratorSeed(seed, 20688817, -21014330, 30001) == 62107075293514);
        REQUIRE(getDecoratorSeed(seed, -20128074, 7672530, 30001) == 206741527306163);
        seed = 4917349648921975370;
        REQUIRE(getDecoratorSeed(seed, -11362826, 11542523, 30001) == 121265705332649);
        REQUIRE(getDecoratorSeed(seed, 3363792, 1936537, 30001) == 178611928168781);
        REQUIRE(getDecoratorSeed(seed, 21237450, -4339542, 30001) == 166135716578382);
        REQUIRE(getDecoratorSeed(seed, 13366007, 3299662, 30001) == 124694995517163);
        REQUIRE(getDecoratorSeed(seed, -703110, 10173674, 30001) == 71988513566814);
    }
}

TEST_CASE("Validating seeds works", "[checkSeed]") {
    int64_t seed = -4872636734044769429;
    REQUIRE(canGenerateTreasure(seed, -28, -73));
    REQUIRE(!canGenerateTreasure(seed, -28, -72));
    REQUIRE(checkSeed(seed, -28, -73));
    REQUIRE(!checkSeed(seed, -28, -72));
}
