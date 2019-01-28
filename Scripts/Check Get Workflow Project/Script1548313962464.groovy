import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper
import static org.assertj.core.api.Assertions.*

response = WS.sendRequest(findTestObject('Get Workflow Project', [('BASE_URL') : GlobalVariable.BASE_URL, ('API_VERSION') : GlobalVariable.API_VERSION
            , ('BUSINESS_TAG_ID') : GlobalVariable.BUSINESS_TAG_ID, ('WFID') : 'WFID_'+GlobalVariable.WFID, ('LOGIN_TOKEN') : GlobalVariable.LOGIN_TOKEN]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'error', 'false')

WS.containsString(response, 'data', false)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

def elementsSize = jsonResponse.data.elements.size()

assert elementsSize > 0

for (def index : (0..elementsSize)) {
	if(jsonResponse.data.elements[index] && jsonResponse.data.elements[index].cardtype == 'webviewcard' && jsonResponse.data.elements[index].title != 'TOP') { 
		def tmpTag = jsonResponse.data.elements[index].allTags[1]
		def tag1 = tmpTag.substring(tmpTag.indexOf(':') + 1,tmpTag.length())
		def tag2 = jsonResponse.data.elements[index].allTags[0]
		def tag = tag1 + ':' + tag2
		GlobalVariable.DEPARTMENTS.add(tag)
	}
}

