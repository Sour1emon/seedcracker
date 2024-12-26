//
//  main.cpp
//  seedcracker
//
//  Created by Isaac Bess on 12/24/24.
//

#include "seedcracker.hpp"

#include <iostream>
#include <iostream>
#include <string>
#include <sstream>
#include <iomanip>
#include <chrono>
#include <vector>
#include <thread>
#include <array>
#include <optional>
#include <pthread.h>

using namespace std::chrono;

std::string formatNum(uint64_t num) {
    std::ostringstream oss;
    if (num < 10000) {
        oss << num;
    } else if (num < 1000000) {
        oss << std::fixed << std::setprecision(0) << (num / 1000.0) << "k";
    } else if (num < 10000000) {
        oss << std::fixed << std::setprecision(1) << (num / 1000000.0) << "M";
    } else if (num < 1000000000) {
        oss << std::fixed << std::setprecision(0) << (num / 1000000.0) << "M";
    } else if (num < 10000000000) {
        oss << std::fixed << std::setprecision(1) << (num / 1000000000.0) << "B";
    } else if (num < 1000000000000) {
        oss << std::fixed << std::setprecision(0) << (num / 1000000000.0) << "B";
    } else if (num < 10000000000000) {
        oss << std::fixed << std::setprecision(1) << (num / 1000000000000.0) << "T";
    } else if (num < 1000000000000000) {
        oss << std::fixed << std::setprecision(0) << (num / 1000000000000.0) << "T";
    } else {
        oss << std::fixed << std::setprecision(2) << (num / 1000000000000000.0) << "Q";
    }
    return oss.str();
}

const uint64_t MAX_SEEDS = 1ul << 32;

const int THREAD_COUNT = 16;

static std::array<std::thread, THREAD_COUNT> THREADS;

void checkSeedsThread(int tIdx, uint64_t *seedsScanned, bool *shouldStop) {
    Generator g;
    setupGenerator(&g, MC_1_16_5, 0);
    for (uint64_t s = tIdx; s < MAX_SEEDS; s += THREAD_COUNT) {
        if (*shouldStop) {
            break;
        }
        uint64_t lower48 = s + uint64_t(-4872636734044769429) - MAX_SEEDS + 1;
        if (checkSeed(lower48, &g, CHUNK_X, CHUNK_Z)) {
            std::cout << "Found Seed: " << (int64_t) lower48 << std::endl;
            *seedsScanned = s;
            *shouldStop = true;
            break;
        };
    }
}

int main () {
    uint64_t seedsScanned = MAX_SEEDS;
    bool shouldStop = false;
    auto start = high_resolution_clock::now();
    for (int tIdx = 0; tIdx < THREAD_COUNT; tIdx++) {
        std::thread thread(checkSeedsThread, tIdx, &seedsScanned, &shouldStop);
        THREADS[tIdx] = std::move(thread);
    }
    for (int tIdx = 0; tIdx < THREAD_COUNT; tIdx++) {
        std::thread *thread = &THREADS[tIdx];
        thread->join();
    }
    auto stop = high_resolution_clock::now();
    
    auto elapsed = stop - start;
    
    std::cout << "Stats:" << std::endl;
    
    std::cout << formatNum(seedsScanned) << " seeds scanned in " << std::fixed << std::setprecision(4) << (double) duration_cast<microseconds>(elapsed).count() / 1000.0 << "ms" << std::endl;
    
    std::cout << formatNum(((double) seedsScanned / ((double) duration_cast<microseconds>(elapsed).count() / (1000.0 * 1000.0)))) << " seeds/s" << std::endl;
    
    std::cout << formatNum((uint64_t) ((double) seedsScanned / ((double) duration_cast<microseconds>(elapsed).count() / (1000.0 * 1000.0 * 3600.0)))) << " seeds/h" << std::endl;
    
    std::cout << std::fixed << std::setprecision(4) << ((double) duration_cast<nanoseconds>(elapsed).count() / (double) seedsScanned) << "ns per seed" << std::endl;
    
    std::cout << std::fixed << std::setprecision(4) << std::pow(2.0, 48.0) / (((double) seedsScanned / ((double) duration_cast<microseconds>(elapsed).count() / (1000.0 * 1000.0 * 3600.0)))) << " hours for checking all seeds" << std::endl;
}
