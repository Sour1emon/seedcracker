#define NS_PRIVATE_IMPLEMENTATION
#define CA_PRIVATE_IMPLEMENTATION
#define MTL_PRIVATE_IMPLEMENTATION
#include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>
#include <dispatch/dispatch.h>

#include <iostream>
#include <string>
#include <sstream>
#include <iomanip>
#include <chrono>
using namespace std::chrono;

#include <mach-o/dyld.h>
#include <limits.h>
#include <stddef.h>

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

constexpr uint64_t MAX_SEED = 1ul << 36;

const char* GPU_TRACE_PATH = "/Users/isaac/seedcracker/seedcracker/profile.gputrace";

const bool SHOULD_CAPTURE = false;

int main() {
    auto device = MTL::CreateSystemDefaultDevice();
    
    if (!device) {
        std::cerr << "Metal is not supported on this device." << std::endl;
        return -1;
    }
    
    auto commandQueue = device->newCommandQueue();
    
    auto defaultLibrary = device->newDefaultLibrary();
    
    auto functionConstants = MTL::FunctionConstantValues::alloc()->init();
    functionConstants->setConstantValue(&MAX_SEED, MTL::DataTypeULong, NS::UInteger(0));
    
    NS::Error* error = nullptr;
    auto kernelFunction = defaultLibrary->newFunction(NS::String::string("find_seed", NS::UTF8StringEncoding), functionConstants, &error);
    
    if (error) {
        std::cerr << "Error creating function: "
        << error->localizedDescription()->utf8String() << std::endl;
        return -1;
    }
    
    error = nullptr;
    auto pipelineState = device->newComputePipelineState(kernelFunction, &error);
    
    if (error) {
        std::cerr << "Error creating compute pipeline: "
        << error->localizedDescription()->utf8String() << std::endl;
        return -1;
    }
    
    auto resultBuffer = device->newBuffer(sizeof(uint64_t) * 2, MTL::ResourceStorageModeShared);
    auto shouldExitBuffer = device->newBuffer(sizeof(bool), MTL::ResourceStorageModeShared);
    MTL::CaptureManager *captureManager;
    if (SHOULD_CAPTURE) {
        
        MTL::CaptureManager *captureManager = MTL::CaptureManager::sharedCaptureManager();
        MTL::CaptureDescriptor *captureDescriptor = MTL::CaptureDescriptor::alloc();
        captureDescriptor->init();
        captureDescriptor->setCaptureObject(device);
        captureDescriptor->setDestination(MTL::CaptureDestination::CaptureDestinationGPUTraceDocument);
        const NS::String *filePath = NS::String::string(GPU_TRACE_PATH, NS::StringEncoding::UTF8StringEncoding);
        captureDescriptor->setOutputURL(NS::URL::fileURLWithPath(filePath));
        
        error = nullptr;
        
        captureManager->startCapture(captureDescriptor, &error);
        
        if (error) {
            std::cerr << "Error starting capture: "
            << error->localizedDescription()->utf8String() << std::endl;
            return -1;
        }
    }
    
    auto commandBuffer = commandQueue->commandBuffer();
    auto computeEncoder = commandBuffer->computeCommandEncoder();
    
    computeEncoder->setComputePipelineState(pipelineState);
    computeEncoder->setBuffer(resultBuffer, 0, 0);
    computeEncoder->setBuffer(shouldExitBuffer, 0, 1);
    
    auto max_threads_per_group = pipelineState->maxTotalThreadsPerThreadgroup();
    auto num_thread_groups = (MAX_SEED + max_threads_per_group - 1) / max_threads_per_group;
    
    MTL::Size threadsPerThreadgroup = MTL::Size::Make(max_threads_per_group, 1, 1);
    MTL::Size threadgroupCount = MTL::Size::Make(num_thread_groups, 1, 1);
    
    computeEncoder->dispatchThreads(threadgroupCount, threadsPerThreadgroup);
    computeEncoder->endEncoding();
    
    std::cout << "Starting seed cracking on gpu" << std::endl;
    
    commandBuffer->commit();
    
    auto start = high_resolution_clock::now();
    
    commandBuffer->waitUntilCompleted();
    
    
    if (SHOULD_CAPTURE) {
        captureManager->stopCapture();
    }
    
    auto stop = high_resolution_clock::now();
    
    auto elapsed = stop - start;
    
    uint64_t seedsScanned = MAX_SEED;
    
    std::cout << "Stats:" << std::endl;
    
    std::cout << formatNum(seedsScanned) << " seeds scanned in " << std::fixed << std::setprecision(4) << (double) duration_cast<microseconds>(elapsed).count() / 1000.0 << "ms" << std::endl;
    
    std::cout << formatNum(((double) seedsScanned / ((double) duration_cast<microseconds>(elapsed).count() / (1000.0 * 1000.0)))) << " seeds/s" << std::endl;
    
    std::cout << formatNum((uint64_t) ((double) seedsScanned / ((double) duration_cast<microseconds>(elapsed).count() / (1000.0 * 1000.0 * 3600.0)))) << " seeds/h" << std::endl;
    
    std::cout << std::fixed << std::setprecision(4) << ((double) duration_cast<nanoseconds>(elapsed).count() / (double) seedsScanned) << "ns per seed" << std::endl;
    
    std::cout << std::fixed << std::setprecision(4) << std::pow(2.0, 48.0) / (((double) seedsScanned / ((double) duration_cast<microseconds>(elapsed).count() / (1000.0 * 1000.0 * 3600.0)))) << " hours for checking all seeds" << std::endl;
    
    uint64_t *resultData = static_cast<uint64_t *>(resultBuffer->contents());
    
    if (resultData[1]) {
        std::cout << "Found seed: " << resultData[0] << std::endl;
    }
    
    resultBuffer->release();
    computeEncoder->release();
    commandBuffer->release();
    pipelineState->release();
    kernelFunction->release();
    defaultLibrary->release();
    commandQueue->release();
    device->release();
    
    return 0;
}
